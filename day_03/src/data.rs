use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Num(pub u32);

impl FromStr for Num {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s
            .chars()
            .map(|c| if c == '0' { 0b0 } else { 0b1 })
            .fold(0, |acc, b| (acc << 1) | b);

        Ok(Num(num))
    }
}

pub fn gamma_rate(nums: &Vec<Num>, bits: u32) -> u32 {
    let mut gamma_rate: Option<u32> = None;

    for idx in 0..bits {
        let mut zero_count = 0;
        let mut one_count = 0;

        for num in nums {
            // When idx=0 and bits=5, this will be 0b10000, then 0b01000, 0b00100, etc. Do a bitwise
            // AND with the number. If the resulting number is non-zero, then there is a 1 at the
            // idx-th bit "column".
            let bitmask = 0b1 << (bits - 1 - idx);
            let result = bitmask & num.0;

            if result == 0 {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }

        // For this "column", add a 0 if there were more 0's overall
        let bit_to_add = if zero_count > one_count { 0 } else { 1 };

        if let Some(mut gr) = gamma_rate {
            // Shift the binary number one left, then set the "new" right-most bit to bit_to_add
            gr = gr << 1;
            gr += bit_to_add;
            gamma_rate = Some(gr);
        } else {
            gamma_rate = Some(bit_to_add);
        }
    }

    gamma_rate.unwrap()
}

pub fn oxygen_rating(nums: Vec<Num>, bits: u32) -> u32 {
    find_rating(nums, bits, Mode::KeepMostCommonTiebreakerOne)
}

pub fn co2_rating(nums: Vec<Num>, bits: u32) -> u32 {
    find_rating(nums, bits, Mode::KeepLeastCommonTiebreakerZero)
}

enum Mode {
    KeepLeastCommonTiebreakerZero,
    KeepMostCommonTiebreakerOne,
}

// Most common value (0 or 1) in the current bit position, and keep only numbers with that bit in
// that position. If 0 and 1 are equally common, keep values with the Tiebreaker value in the
// position being considered. Repeat until one number remains.
fn find_rating(mut nums: Vec<Num>, bits: u32, tiebreaker: Mode) -> u32 {
    use Mode::*;

    let mut idx = 0;

    while nums.len() > 1 {
        // Same as in gamma_rate
        let bitmask = 0b1 << (bits - 1 - idx);

        let zero_count = nums
            .iter()
            .filter(|Num(n)| (n & bitmask).count_ones() == 0)
            .count();

        let one_count = nums.len() - zero_count;

        // Logic depends on the "mode"
        let keep_if_bit_is = match tiebreaker {
            KeepLeastCommonTiebreakerZero => !(zero_count <= one_count) as u32, // 0
            KeepMostCommonTiebreakerOne => (one_count >= zero_count) as u32,    // 1
        };

        nums.retain(|Num(n)| (n & bitmask).count_ones() == keep_if_bit_is);
        idx += 1;
    }

    nums[0].0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_num_from_str() {
        assert_eq!(get_test_input::<Num>()[0], Num(0b00100));
        assert_eq!(get_test_input::<Num>()[0], Num(4));
    }

    #[test]
    fn test_gamma_rate() {
        assert_eq!(gamma_rate(&get_test_input::<Num>(), 5), 0b10110);
        assert_eq!(gamma_rate(&get_test_input::<Num>(), 5), 22);
        assert_eq!(
            gamma_rate(&vec![Num(0b111111000001), Num(0b111111000001)], 12),
            0b111111000001
        );
    }

    #[test]
    fn test_oxygen_rating() {
        assert_eq!(oxygen_rating(get_test_input::<Num>(), 5), 0b10111);
    }

    #[test]
    fn test_co2_rating() {
        assert_eq!(co2_rating(get_test_input::<Num>(), 5), 0b01010);
    }
}
