mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 09");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// Find the low points: lower than any of its adjacent locations (no diagonals). Risk level of a low
// point is 1 plus its height. What is the sum of the risk levels?
fn part_1(lines: Vec<String>) -> u32 {
    HeightMap::new(lines)
        .low_points()
        .map(|(_coord, height)| height + 1)
        .sum()
}

// A basin is all locations that eventually flow downward to a single low point. Every low point has
// a basin. Locations of height 9 do not count as being in any basin, and all other locations will
// always be part of exactly one basin. What do you get if you multiply together the sizes of the
// three largest basins?
fn part_2(lines: Vec<String>) -> usize {
    let height_map = HeightMap::new(lines);

    let mut basin_sizes = height_map
        .low_points()
        .map(|(coord, _height)| height_map.basin_size(coord))
        .collect::<Vec<usize>>();

    basin_sizes.sort();

    basin_sizes
        .iter()
        .rev()
        .take(3)
        .fold(1, |total, size| total * size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 1134);
    }
}
