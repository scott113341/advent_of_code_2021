use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct InputLine {
    pub signals: Vec<SignalPattern>,
    pub outputs: Vec<SignalPattern>,
}

impl InputLine {
    // Yikes
    pub fn solve(&self) -> usize {
        let mut solves = BTreeMap::new();

        let mut seg_counts: HashMap<char, usize> = HashMap::new();
        for sig in self.signals.iter() {
            for seg in sig.segments.iter().filter(|&&c| c != '\u{0}') {
                *seg_counts.entry(*seg).or_insert(0) += 1;
            }
        }

        let seven_chars = self
            .signals
            .iter()
            .find(|s| s.segment_count == 3)
            .unwrap()
            .chars();
        let one_chars = self
            .signals
            .iter()
            .find(|s| s.segment_count == 2)
            .unwrap()
            .chars();
        let four_chars = self
            .signals
            .iter()
            .find(|s| s.segment_count == 4)
            .unwrap()
            .chars();

        // Solve 'a': the char in 7 that's not in 1
        solves.insert('a', *seven_chars.difference(&one_chars).nth(0).unwrap());

        // Solve 'f': the char in 9 numbers
        solves.insert(
            'f',
            *seg_counts
                .iter()
                .find(|(_seg, &count)| count == 9)
                .unwrap()
                .0,
        );

        // Solve 'b': the char in 6 numbers
        solves.insert(
            'b',
            *seg_counts
                .iter()
                .find(|(_seg, &count)| count == 6)
                .unwrap()
                .0,
        );

        // Solve 'e': the char in 4 numbers
        solves.insert(
            'e',
            *seg_counts
                .iter()
                .find(|(_seg, &count)| count == 4)
                .unwrap()
                .0,
        );

        // Solve 'c': the unsolved char in 7
        solves.insert('c', {
            let solved = solves.clone().into_values().collect();
            *seven_chars.difference(&solved).nth(0).unwrap()
        });

        // Solve 'd': the unsolved char in 4
        solves.insert('d', {
            let solved = solves.clone().into_values().collect();
            *four_chars.difference(&solved).nth(0).unwrap()
        });

        // Solve 'g': the final unsolved char
        solves.insert('g', {
            let solved = solves.clone().into_values().collect();
            *('a'..='g')
                .collect::<HashSet<char>>()
                .difference(&solved)
                .nth(0)
                .unwrap()
        });

        // When given "cf", returns the solved mapping, like {'a', 'b'}
        let solves_for = |chars: &str| -> BTreeSet<char> {
            solves
                .iter()
                .filter(|(og_char, _)| chars.contains(**og_char))
                .map(|(_, solved_char)| *solved_char)
                .collect()
        };

        // Construct a map of (set of solved chars) => (digit)
        // BTreeMap<{'b', 'c', 'a', ...}, '7'}>
        let mut solved_numbers: BTreeMap<BTreeSet<char>, char> = BTreeMap::new();
        solved_numbers.insert(solves_for("abcefg"), '0');
        solved_numbers.insert(solves_for("cf"), '1');
        solved_numbers.insert(solves_for("acdeg"), '2');
        solved_numbers.insert(solves_for("acdfg"), '3');
        solved_numbers.insert(solves_for("bcdf"), '4');
        solved_numbers.insert(solves_for("abdfg"), '5');
        solved_numbers.insert(solves_for("abdefg"), '6');
        solved_numbers.insert(solves_for("acf"), '7');
        solved_numbers.insert(solves_for("abcdefg"), '8');
        solved_numbers.insert(solves_for("abcdfg"), '9');

        self.outputs
            .iter()
            .map(|o| solved_numbers.get(&o.chars()).unwrap())
            .collect::<String>()
            .parse()
            .unwrap()
    }
}

impl FromStr for InputLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signals_str, outputs_str) = s.split_once(" | ").unwrap();

        Ok(InputLine {
            signals: signals_str.split(' ').map(|s| s.parse().unwrap()).collect(),
            outputs: outputs_str.split(' ').map(|s| s.parse().unwrap()).collect(),
        })
    }
}

#[derive(Default, Debug)]
pub struct Solution {
    pub a: Option<char>,
    pub b: Option<char>,
    pub c: Option<char>,
    pub d: Option<char>,
    pub e: Option<char>,
    pub f: Option<char>,
    pub g: Option<char>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct SignalPattern {
    segments: [char; 7],
    segment_count: usize,
}

impl SignalPattern {
    pub fn ez_num(&self) -> Option<usize> {
        match self.segment_count {
            2 => Some(1),
            4 => Some(4),
            3 => Some(7),
            7 => Some(8),
            _ => None,
        }
    }

    pub fn chars(&self) -> BTreeSet<char> {
        self.segments
            .clone()
            .into_iter()
            .take(self.segment_count)
            .collect()
    }
}

impl FromStr for SignalPattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments: [char; 7] = Default::default();
        let mut segment_count = 0;

        for (idx, char) in s.chars().enumerate() {
            segments[idx] = char;
            segment_count += 1;
        }

        Ok(SignalPattern {
            segments,
            segment_count,
        })
    }
}

#[cfg(test)]
macro_rules! signal_pattern {
    ($($char_:expr),*) => {
        {
            #[allow(unused_mut)]
            let mut segments: [char; 7] = Default::default();
            #[allow(unused_mut)]
            let mut segment_count = 0;
            signal_pattern!(@recurse segments, segment_count, 0usize, $($char_),*);
            SignalPattern { segments, segment_count }
        }
    };

    // Recursively set chars and increment the count. The separate $head_char and $tail_char,*
    // captures cause each pass through this rule to "shift" off the 0th char.
    (@recurse $seg:ident, $seg_count:ident, $idx:expr, $head_char:expr, $($tail_char:expr),*) => {
        $seg[$idx] = $head_char;
        $seg_count += 1;
        signal_pattern!(@recurse $seg, $seg_count, $idx + 1usize, $($tail_char),*);
    };

    (@recurse $seg:ident, $seg_count:ident, $idx:expr, $head_char:expr) => {
        $seg[$idx] = $head_char;
        $seg_count += 1;
    };

    // This gets called at the last recursion step, when there's no more tail_chars
    (@recurse $seg:ident, $seg_count:ident, $idx:expr, $($tail_char:expr),*) => {};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_input_line_from_str() {
        let input_line = get_test_input::<InputLine>().remove(0);
        assert_eq!(input_line.signals[0], signal_pattern!['b', 'e']);
    }

    #[test]
    fn test_signal_pattern_macro() {
        assert_eq!(
            signal_pattern![],
            SignalPattern {
                segments: Default::default(),
                segment_count: 0,
            }
        );

        assert_eq!(
            signal_pattern!['a'],
            SignalPattern {
                segments: ['a', '\u{0}', '\u{0}', '\u{0}', '\u{0}', '\u{0}', '\u{0}'],
                segment_count: 1,
            }
        );

        assert_eq!(
            signal_pattern!['b', 'e'],
            SignalPattern {
                segments: ['b', 'e', '\u{0}', '\u{0}', '\u{0}', '\u{0}', '\u{0}'],
                segment_count: 2,
            }
        );

        assert_eq!(
            signal_pattern!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            SignalPattern {
                segments: ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
                segment_count: 7,
            }
        );
    }

    #[test]
    fn test_input_line_solve() {
        let input_line: InputLine =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
                .parse()
                .unwrap();

        assert_eq!(input_line.solve(), 5353);
    }
}
