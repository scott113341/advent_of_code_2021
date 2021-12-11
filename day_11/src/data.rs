use std::collections::{BTreeMap, BTreeSet};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct Coord {
    pub y: isize,
    pub x: isize,
}

#[derive(Eq, PartialEq, Debug)]
pub struct OctoGrid {
    octos: BTreeMap<Coord, usize>,
    step: usize,
    flashes: usize,
}

impl OctoGrid {
    pub fn new(initial: Vec<usize>) -> OctoGrid {
        let mut octos = BTreeMap::new();

        for (idx, energy) in initial.into_iter().enumerate() {
            let coord = Coord {
                x: idx as isize % 10,
                y: idx as isize / 10,
            };
            octos.insert(coord, energy);
        }

        OctoGrid {
            octos,
            step: 0,
            flashes: 0,
        }
    }

    pub fn flashes(&self) -> usize {
        self.flashes
    }

    pub fn step(&self) -> usize {
        self.step
    }

    // Increase each energy level by 1. Then, any octopus with energy level >9 flashes. This
    // increases the energy level of all adjacent octopuses by 1, including diagonals. If this
    // causes an octopus to have an energy level greater than 9, it also flashes. Any octopus that
    // flashed during this step has its energy level set to 0.
    pub fn step_forward(&mut self) {
        for (_coord, energy) in self.octos.iter_mut() {
            *energy += 1;
        }

        // Flash any octopi with over 9 energy (which increases adjacent energies). Keep doing this
        // until a steady state is reached.
        let mut flashed: BTreeSet<Coord> = BTreeSet::new();
        loop {
            let to_flash: Vec<_> = self
                .octos
                .clone()
                .into_iter()
                .filter(|(coord, energy)| *energy > 9 && !flashed.contains(coord))
                .collect();

            if !to_flash.is_empty() {
                for (coord, _energy) in to_flash {
                    self.increase_energy_around(&coord);
                    flashed.insert(coord);
                }
            } else {
                break;
            }
        }

        // Reset any octopus that flashed to 0 energy
        for coord in flashed.iter() {
            *self.octos.get_mut(coord).unwrap() = 0;
        }

        self.step += 1;
        self.flashes += flashed.len();
    }

    fn increase_energy_around(&mut self, coord: &Coord) {
        for dx in -1..=1 {
            for dy in -1..=1 {
                // Skip increasing the given Coord's energy
                if dx == 0 && dy == 0 {
                    continue;
                }

                let adjacent_coord = Coord {
                    x: coord.x + dx,
                    y: coord.y + dy,
                };
                self.octos.entry(adjacent_coord).and_modify(|e| *e += 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::get_test_input;

    #[test]
    fn test_octo_grid_step_forward() {
        let mut octo_grid = get_test_input();

        octo_grid.step_forward();
        assert_eq!(octo_grid.flashes(), 0);

        octo_grid.step_forward();
        assert_eq!(octo_grid.flashes(), 35);

        for _ in 0..8 {
            octo_grid.step_forward();
        }
        assert_eq!(octo_grid.flashes(), 204);

        for _ in 0..90 {
            octo_grid.step_forward();
        }
        assert_eq!(octo_grid.flashes(), 1656);
    }
}
