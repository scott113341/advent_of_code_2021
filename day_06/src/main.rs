mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 06");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// How many lanternfish would there be after 80 days?
fn part_1(initial_fish: Vec<u8>) -> usize {
    let fish_state: FishState = initial_fish.into();
    fish_state.skip(79).next().unwrap().fish_count()
}

// How many lanternfish would there be after 256 days?
fn part_2(initial_fish: Vec<u8>) -> usize {
    let fish_state: FishState = initial_fish.into();
    fish_state.skip(255).next().unwrap().fish_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 5934);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 26984457539);
    }
}
