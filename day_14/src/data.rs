use std::collections::BTreeMap;

pub type Chain = String;
pub type Rules = BTreeMap<(char, char), char>;

#[derive(Eq, PartialEq, Debug)]
pub struct Polymer {
    chain: Chain,
    rules: Rules,
}

impl Polymer {
    pub fn new(chain: &Chain, rules: &Rules) -> Polymer {
        Polymer {
            chain: chain.clone(),
            rules: rules.clone(),
        }
    }

    pub fn parse_new(lines: &Vec<String>) -> Polymer {
        let chain = lines[0].clone();

        let rules = lines
            .into_iter()
            .filter_map(|rl| {
                if rl.contains("->") {
                    let chars: Vec<char> = rl.chars().collect();
                    Some(((chars[0], chars[1]), chars[6]))
                } else {
                    None
                }
            })
            .collect();

        Polymer { chain, rules }
    }

    pub fn chain(&self) -> &Chain {
        &self.chain
    }

    pub fn rules(&self) -> &Rules {
        &self.rules
    }

    pub fn react(&mut self) {
        let mut new_chain = String::with_capacity(self.chain.len() * 2);
        let mut chars = self.chain.chars().peekable();

        while let Some(this_char) = chars.next() {
            new_chain.push(this_char);

            if let Some(next_char) = chars.peek() {
                let pair = (this_char, *next_char);
                if let Some(insert_char) = self.rules.get(&pair) {
                    new_chain.push(*insert_char);
                }
            }
        }

        self.chain = new_chain;
    }

    pub fn react_times(&mut self, times: usize) {
        for _ in 1..=times {
            self.react();
        }
    }

    // Quantity of the most common element minus quantity of the least common element
    pub fn delta(&self) -> usize {
        let counts = self.chemical_counts();
        counts.values().max().unwrap() - counts.values().min().unwrap()
    }

    pub fn chemical_counts(&self) -> BTreeMap<char, usize> {
        let mut counts = BTreeMap::new();

        for c in self.chain.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_polymer_new() {
        let polymer = Polymer::parse_new(&get_test_input::<String>());
        assert_eq!(polymer.chain, "NNCB".to_string());
        assert_eq!(polymer.rules.get(&('H', 'H')), Some(&'N'));
        assert_eq!(polymer.rules.get(&('Q', 'Q')), None);
    }

    #[test]
    fn test_polymer_react() {
        let mut polymer = Polymer::parse_new(&get_test_input::<String>());
        assert_eq!(polymer.chain, "NNCB".to_string());
        polymer.react();
        assert_eq!(polymer.chain, "NCNBCHB".to_string());
        polymer.react();
        assert_eq!(polymer.chain, "NBCCNBBBCBHCB".to_string());
        polymer.react();
        assert_eq!(polymer.chain, "NBBBCNCCNBBNBNBBCHBHHBCHB".to_string());
    }
}
