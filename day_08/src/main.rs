// #![feature(trace_macros)]
// trace_macros!(true);

mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 08");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// Each entry consists of ten unique signal patterns, a | delimiter, and finally the four digit
// output value. Within an entry, the same wire/segment connections are used. For now, focus on the
// easy digits that can be decoded by looking at the number of segments (1, 4, 7, 8). In the output
// values, how many times do these digits appear?
fn part_1(input_lines: Vec<InputLine>) -> usize {
    let mut count = 0;

    for input_line in input_lines.iter() {
        for signal_pattern in input_line.outputs.iter() {
            if signal_pattern.ez_num().is_some() {
                count += 1;
            }
        }
    }

    count
}

// For each entry, determine all of the wire/segment connections and decode the four-digit output
// values. What do you get if you add up all of the output values?
fn part_2(input_lines: Vec<InputLine>) -> usize {
    input_lines.iter().map(|il| il.solve()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 26);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 61229);
    }
}
