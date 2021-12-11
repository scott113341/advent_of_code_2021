mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 11");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// How many total flashes are there after 100 steps?
fn part_1(mut octo_grid: OctoGrid) -> usize {
    for _ in 1..=100 {
        octo_grid.step_forward();
    }

    octo_grid.flashes()
}

// What is the first step during which all octopuses flash?
fn part_2(mut octo_grid: OctoGrid) -> usize {
    let mut prev_flash_count = 0;

    loop {
        octo_grid.step_forward();
        if octo_grid.flashes() == prev_flash_count + 100 {
            return octo_grid.step();
        } else {
            prev_flash_count = octo_grid.flashes();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 1656);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 195);
    }
}
