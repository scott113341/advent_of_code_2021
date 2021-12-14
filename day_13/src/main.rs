mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 13");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2:");
    for line in part_2(get_input()).lines() {
        println!("    {}", line);
    }
}

// How many dots are visible after completing just the first fold instruction?
fn part_1(lines: Vec<String>) -> usize {
    let mut paper = Paper::new(&lines);
    paper.fold();
    paper.dots.len()
}

// Finish folding the paper. The manual says the code is always eight capital letters. What is it?
fn part_2(lines: Vec<String>) -> String {
    let mut paper = Paper::new(&lines);
    while !paper.folds.is_empty() {
        paper.fold();
    }
    paper.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 17);
    }
}
