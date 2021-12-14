mod data;
mod input;

use data::*;
use input::*;
use std::collections::BTreeMap;

fn main() {
    println!("day: 14");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// Apply 10 steps of pair insertion to the polymer, and return the delta
fn part_1(lines: Vec<String>) -> usize {
    let mut polymer = Polymer::parse_new(&lines);
    polymer.react_times(10);
    polymer.delta()
}

// Apply 40 steps of pair insertion to the polymer, and return the delta. I tried finding a closed-
// form solution, or some math trick to shortcut computation, but didn't find anything. So, this
// is kind of brute-force; for each possible pair, do 20 iterations and save the final character
// counts. Then, do 20 iterations with the actual input. For each pair in that real 20th iteration,
// extrapolate final counts?
fn part_2(lines: Vec<String>) -> usize {
    let mut polymer = Polymer::parse_new(&lines);
    let mut final_counts: BTreeMap<char, usize> = BTreeMap::new();

    // For each possible two-chemical pair, compute the final chemical counts after 20 reactions
    let mut pair_counts_after_20: BTreeMap<(char, char), BTreeMap<char, usize>> = BTreeMap::new();
    for (c1, c2) in polymer.rules().keys() {
        let pair = String::from_iter([c1, c2]);
        let mut pair_polymer = Polymer::new(&pair, polymer.rules());
        pair_polymer.react_times(20);
        pair_counts_after_20.insert((*c1, *c2), pair_polymer.chemical_counts());
    }

    // React the real polymer halfway; we'll "cheat" on the remaining 20 iterations using the
    // pre-computed counts.
    polymer.react_times(20);

    // For each pair in the halfway-reacted polymer, add the pre-computed counts for what it'll
    // eventually evolve into.
    let halfway_chars: Vec<char> = polymer.chain().chars().collect();
    let mut idx = 0;
    while let Some(c2) = halfway_chars.get(idx + 1) {
        let c1 = halfway_chars.get(idx).unwrap();
        let pair = (*c1, *c2);
        let counts_after_20_more = pair_counts_after_20.get(&pair).unwrap();

        // Add the counts
        for (c, count) in counts_after_20_more {
            *final_counts.entry(*c).or_insert(0) += count;
        }

        idx += 1;
    }

    // Each chemical besides the first and last in the 20th iteration gets double-counted because
    // "ABCD" gets extrapolated and counted as "A...B", "B...C", "C...D". Compensate for that.
    final_counts.entry(halfway_chars[0]).and_modify(|e| *e += 1);
    final_counts
        .entry(halfway_chars[halfway_chars.len() - 1])
        .and_modify(|e| *e += 1);
    for c in halfway_chars.iter() {
        final_counts.entry(*c).and_modify(|e| *e -= 1);
    }

    // Final delta calculation
    final_counts.values().max().unwrap() - final_counts.values().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 1588);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 2188189693529);
    }
}
