mod data;
mod input;

use data::*;
use input::*;
use std::cell::RefCell;

fn main() {
    println!("day: 04");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// Find winning (no diagonals) bingo Board. What's the score?
fn part_1(lines: Vec<String>) -> usize {
    let numbers_to_call = parse_numbers_to_call(&lines);
    let mut boards = parse_boards(&lines);

    for n in numbers_to_call {
        for board in boards.iter_mut() {
            if board.mark_number(n) {
                return board.score().unwrap();
            }
        }
    }

    panic!("No boards won!");
}

// Score of the Board that wins last
fn part_2(lines: Vec<String>) -> usize {
    let numbers_to_call = parse_numbers_to_call(&lines);

    // Wrap each Board in a RefCell so that we can solve the Board (requires mut) while they're
    // being sorted.
    let mut boards = parse_boards(&lines)
        .into_iter()
        .map(|b| RefCell::new(b))
        .collect::<Vec<_>>();

    // Sort by the number of moves each Board takes to win
    boards.sort_by_cached_key(|board| {
        for (move_idx, n) in numbers_to_call.iter().enumerate() {
            if board.borrow_mut().mark_number(*n) {
                return move_idx + 1;
            }
        }

        panic!("Board never won!");
    });

    let winning_board = boards.last().unwrap().borrow();
    winning_board.score().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 4512);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 1924);
    }
}
