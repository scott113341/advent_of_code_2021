mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 03");
    println!("  part 1: {}", part_1(get_input(), 12));
    println!("  part 2: {}", part_2(get_input(), 12));
}

// Each bit in the gamma rate can be determined by finding the most common bit in the corresponding
// position. Epsilon rate is the least common bit in each position. What is the power consumption,
// defined by gamma * epsilon?
fn part_1(nums: Vec<Num>, bits: u32) -> u32 {
    let gamma_rate = gamma_rate(&nums, bits);

    // To calculate epsilon, take gamma and do a bitwise NOT:
    //   Gamma:   10111
    //   Epsilon: 01000
    // Not so fast! There are actually a bunch of leading zero bits, so the NOT looks like:
    //   Gamma:   00000000000000000010111
    //   Epsilon: 11111111111111111101000
    // So, we build a mask of the rightmost bits we care about, then do a bitwise AND:
    //   Mask:    00000000000000000011111
    //   Epsilon: 00000000000000000001000
    let mask = 2_u32.pow(bits.into()) - 1;
    let epsilon_rate = !gamma_rate & mask;

    // Final answer
    gamma_rate * epsilon_rate
}

fn part_2(nums: Vec<Num>, bits: u32) -> u32 {
    oxygen_rating(nums.clone(), bits) * co2_rating(nums, bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input(), 5), 198);
        assert_eq!(
            part_1(vec![Num(0b111111000001), Num(0b111111000001)], 12),
            0b111101000010111110
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input(), 5), 230);
    }
}
