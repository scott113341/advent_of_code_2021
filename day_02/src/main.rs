mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 02");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// What do you get if you multiply your final horizontal position by your final depth?
fn part_1(commands: Vec<Command>) -> usize {
    let mut position = Position::default();

    for command in commands {
        position.process_command_part_1(&command);
    }

    position.horizontal * position.depth
}

// Same, but commands change "aim" as well.
fn part_2(commands: Vec<Command>) -> usize {
    let mut position = Position::default();

    for command in commands {
        position.process_command_part_2(&command);
    }

    position.horizontal * position.depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 150);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 900);
    }
}
