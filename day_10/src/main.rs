mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 10");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// Find the first illegal character in each corrupted line of the navigation subsystem. What is the
// total syntax error score for those errors?
fn part_1(nav_lines: Vec<NavLine>) -> usize {
    nav_lines.into_iter().map(|nl| nl.error_score()).sum()
}

// Find the completion string for each incomplete line, score the completion strings, and sort the
// scores. What is the middle score?
fn part_2(nav_lines: Vec<NavLine>) -> usize {
    let mut scores: Vec<_> = nav_lines
        .into_iter()
        .filter(|nl| !nl.is_corrupted())
        .map(|nl| nl.completion_score())
        .collect();

    scores.sort();
    *scores.get(scores.len() / 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 26397);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 288957);
    }
}
