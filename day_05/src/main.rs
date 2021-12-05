mod data;
mod input;

use data::*;
use input::*;
use std::collections::HashMap;

fn main() {
    println!("day: 05");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// For now, only consider horizontal and vertical lines of hydrothermal vents. At how many points do
// at least two lines overlap?
fn part_1(vent_lines: Vec<VentLine>) -> usize {
    solve(vent_lines, true)
}

// Same, but consider diagonals
fn part_2(vent_lines: Vec<VentLine>) -> usize {
    solve(vent_lines, false)
}

fn solve(vent_lines: Vec<VentLine>, skip_diagonal: bool) -> usize {
    let mut map: HashMap<Coord, usize> = HashMap::new();

    for vent_line in vent_lines {
        if skip_diagonal && vent_line.is_diagonal() {
            continue;
        }

        for coord in vent_line.coords() {
            let count = map.entry(coord).or_insert(0);
            *count += 1;
        }
    }

    map.values().filter(|&&count| count >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 12);
    }
}
