mod input;

use input::*;

fn main() {
    println!("day: 07");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// Determine the horizontal position that the crabs can align to using the least fuel possible. How
// much fuel must they spend to align to that position?
fn part_1(mut positions: Vec<isize>) -> isize {
    positions.sort();
    let min = positions[0];
    let max = positions[positions.len() - 1];

    // (cost, end_position)
    let mut lowest_config = (isize::MAX, isize::MAX);

    for end_position in min..=max {
        let cost = positions.iter().map(|p| (p - end_position).abs()).sum();
        if cost < lowest_config.0 {
            lowest_config = (cost, end_position);
        }
    }

    lowest_config.0
}

// Same, but each change of 1 step in horizontal position costs 1 more unit of fuel than the last
fn part_2(mut positions: Vec<isize>) -> isize {
    positions.sort();
    let min = positions[0];
    let max = positions[positions.len() - 1];

    // (cost, end_position)
    let mut lowest_config = (isize::MAX, isize::MAX);

    for end_position in min..=max {
        let cost = positions
            .iter()
            .map(|pos| {
                let dist = (pos - end_position).abs();
                dist * (dist + 1) / 2
            })
            .sum();

        if cost < lowest_config.0 {
            lowest_config = (cost, end_position);
        }
    }

    lowest_config.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 37);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 168);
    }
}
