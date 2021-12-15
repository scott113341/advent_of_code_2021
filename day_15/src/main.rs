mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 15");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// What is the lowest total risk of any path from the top left to the bottom right?
fn part_1(lines: Vec<String>) -> usize {
    let cave = Cave::new(&lines);
    cave.lowest_risk_path()
}

// Repeat the map 5x in each direction. Each repeat has each risk increased by 1. Risk levels of 9
// wrap around to 1. What is the lowest risk path?
fn part_2(mut lines: Vec<String>) -> usize {
    let mut expanded_lines = vec![];

    let new_num = |num, add| {
        let mut new_num = num + add;
        if new_num >= 10 {
            new_num = new_num - 10 + 1;
        }
        new_num
    };

    // Repeat horizontally 5x
    for line in lines.iter_mut() {
        let mut expanded_line = String::new();
        let digits = line.chars().map(|c| c.to_digit(10).unwrap()).into_iter();

        for add in 0..=4 {
            for num in digits.clone() {
                expanded_line.push_str(&new_num(num, add).to_string());
            }
        }

        expanded_lines.push(expanded_line.clone());
        *line = expanded_line;
    }

    // Repeat vertically 5x
    for add in 1..=4 {
        for line in lines.iter() {
            let mut expanded_line = String::new();
            let digits = line.chars().map(|c| c.to_digit(10).unwrap()).into_iter();

            for num in digits {
                expanded_line.push_str(&new_num(num, add).to_string());
            }

            expanded_lines.push(expanded_line.clone());
        }
    }

    let cave = Cave::new(&expanded_lines);
    cave.lowest_risk_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input(1)), 40);
        assert_eq!(part_1(get_test_input(2)), 20);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input(1)), 315);
    }
}
