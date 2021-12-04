pub fn parse_numbers_to_call(lines: &Vec<String>) -> Vec<usize> {
    lines
        .first()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

pub fn parse_boards(lines: &Vec<String>) -> Vec<Board> {
    let mut boards = vec![];
    let mut line_idx = 2;
    let mut numbers = [0; 25];
    let mut number_idx = 0;

    while let Some(line) = lines.get(line_idx) {
        // Parse and persist this line's numbers
        let line_numbers = line.split_ascii_whitespace().map(|n| n.parse().unwrap());
        for n in line_numbers {
            numbers[number_idx] = n;
            number_idx += 1;
        }

        // Complete this board once all 25 numbers have been read
        if number_idx == 25 {
            boards.push(Board::new(numbers));
            numbers = [0; 25];
            number_idx = 0;
        }

        line_idx += 1;
    }

    boards
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Board {
    numbers: [usize; 25],       // The actual numbers on the board
    marked_numbers: Vec<usize>, // Numbers marked so far on this Board
    row_states: [usize; 5],     // [0, 2, 1, 1, 0] numbers filled in, T=>B
    column_states: [usize; 5],  // [1, 0, 4, 2, 3] numbers filled in, L=>R
}

impl Board {
    pub fn new(numbers: [usize; 25]) -> Self {
        Board {
            numbers,
            marked_numbers: vec![],
            row_states: [0; 5],
            column_states: [0; 5],
        }
    }

    pub fn mark_number(&mut self, number: usize) -> bool {
        if let Some(idx) = self.numbers.iter().position(|n| *n == number) {
            let row_idx = idx / 5;
            let column_idx = idx % 5;

            self.row_states[row_idx] += 1;
            self.column_states[column_idx] += 1;
            self.marked_numbers.push(number);
        }

        self.is_complete()
    }

    pub fn is_complete(&self) -> bool {
        self.row_states.contains(&5) || self.column_states.contains(&5)
    }

    // Sum of all un-marked numbers, times the final/winning number
    pub fn score(&self) -> Option<usize> {
        if self.is_complete() {
            let unmarked_sum = self
                .numbers
                .iter()
                .filter(|n| !self.marked_numbers.contains(n))
                .sum::<usize>();

            Some(unmarked_sum * self.marked_numbers.last().unwrap())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_parse_boards() {
        assert_eq!(
            parse_boards(&get_test_input())[2],
            Board {
                numbers: [
                    14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2,
                    0, 12, 3, 7,
                ],
                marked_numbers: vec![],
                row_states: [0, 0, 0, 0, 0],
                column_states: [0, 0, 0, 0, 0],
            }
        )
    }

    #[test]
    fn test_board_functionality() {
        let mut board = parse_boards(&get_test_input()).swap_remove(2);

        for n in [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21] {
            assert_eq!(board.mark_number(n), false);
            assert_eq!(board.score(), None);
        }

        assert_eq!(board.mark_number(24), true);
        assert_eq!(board.score(), Some(4512));
    }
}
