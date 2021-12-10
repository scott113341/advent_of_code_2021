use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct NavLine {
    raw: String,
    first_corrupted_char: Option<char>,
    stack: Vec<char>,
}

impl NavLine {
    pub fn is_corrupted(&self) -> bool {
        self.first_corrupted_char.is_some()
    }

    pub fn error_score(&self) -> usize {
        match self.first_corrupted_char {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            Some(_) => unreachable!(),
            None => 0,
        }
    }

    // Start with a total score of 0. Then, for each character, multiply the total score by 5 and
    // then increase the total score by the point value given for the character.
    pub fn completion_score(&self) -> usize {
        // Iterate backwards through the stack, so the order matches how we'd complete the string.
        // For example, if the stack is "<{(", we need ")}>" to complete, in that order.
        self.stack
            .iter()
            .rev()
            .map(|c| match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            })
            .fold(0, |total, char_points| total * 5 + char_points)
    }
}

impl FromStr for NavLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = vec![];
        let mut first_corrupted_char = None;

        for c in s.chars() {
            if "([{<".contains(c) {
                stack.push(c);
            } else if ")]}>".contains(c) {
                let opening = stack.pop();

                let expect_c = match opening {
                    Some('(') => ')',
                    Some('[') => ']',
                    Some('{') => '}',
                    Some('<') => '>',
                    Some(_) => unreachable!(),
                    None => {
                        first_corrupted_char = Some(c);
                        stack.clear();
                        break;
                    }
                };

                if c != expect_c {
                    first_corrupted_char = Some(c);
                    stack.clear();
                    break;
                }
            } else {
                return Err(format!("Unknown char: {}", c));
            }
        }

        Ok(NavLine {
            raw: s.to_string(),
            first_corrupted_char,
            stack,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nav_line_from_str() {
        let s = "[({(<(())[]>[[{[]{<()<>>";
        assert_eq!(
            s.parse(),
            Ok(NavLine {
                raw: s.to_string(),
                first_corrupted_char: None,
                stack: "[({([[{{".chars().collect(),
            })
        );

        let s = "[(()[<>])]({[<{<<[]>>(";
        assert_eq!(
            s.parse(),
            Ok(NavLine {
                raw: s.to_string(),
                first_corrupted_char: None,
                stack: "({[<{(".chars().collect(),
            })
        );

        let s = "{([(<{}[<>[]}>{[]{[(<()>";
        assert_eq!(
            s.parse(),
            Ok(NavLine {
                raw: s.to_string(),
                first_corrupted_char: Some('}'),
                stack: vec![],
            })
        );
    }
}
