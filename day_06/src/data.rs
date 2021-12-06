use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
// HashSet<cycle_day, count>
pub struct FishState(pub(crate) HashMap<u8, usize>);

impl FishState {
    pub fn fish_count(&self) -> usize {
        self.0.values().sum()
    }
}

impl Iterator for FishState {
    type Item = FishState;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_state = HashMap::new();

        for (cycle_day, count) in self.0.drain() {
            if cycle_day == 0 {
                *next_state.entry(6).or_insert(0) += count;
                *next_state.entry(8).or_insert(0) += count;
            } else {
                *next_state.entry(cycle_day - 1).or_insert(0) += count;
            }
        }

        self.0 = next_state.clone();
        Some(FishState(next_state))
    }
}

impl From<Vec<u8>> for FishState {
    fn from(fish: Vec<u8>) -> Self {
        let wtf = HashMap::new();
        let yo = fish.iter().fold(wtf, |mut hm, &d| {
            let count = hm.entry(d).or_insert(0);
            *count += 1;
            hm
        });
        FishState(yo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_fish_state_iterator() {
        let mut fish_state = FishState::from(get_test_input::<u8>());
        assert_eq!(
            fish_state,
            FishState(HashMap::from([(1, 1), (2, 1), (3, 2), (4, 1)]))
        );

        let next_fish_state = fish_state.next().unwrap();
        assert_eq!(
            next_fish_state,
            FishState(HashMap::from([(0, 1), (1, 1), (2, 2), (3, 1)]))
        );

        let next_fish_state = fish_state.next().unwrap();
        assert_eq!(
            next_fish_state,
            FishState(HashMap::from([(0, 1), (1, 2), (2, 1), (6, 1), (8, 1)]))
        );
    }
}
