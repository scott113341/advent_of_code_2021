use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Coord(isize, isize);

#[derive(Eq, PartialEq, Debug)]
pub struct HeightMap {
    pub points: HashMap<Coord, u32>,
}

impl HeightMap {
    pub fn new(lines: Vec<String>) -> HeightMap {
        let mut points = HashMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                points.insert(Coord(x as isize, y as isize), c.to_digit(10).unwrap());
            }
        }

        HeightMap { points }
    }

    pub fn low_points(&self) -> impl Iterator<Item = (&Coord, &u32)> {
        // Points where all adjacent points are higher than this point
        self.points.iter().filter(|(coord, height)| {
            self.adjacent_points(coord).into_iter().all(|point| {
                if let Some((_adj_coord, adj_height)) = point {
                    adj_height > height
                } else {
                    true
                }
            })
        })
    }

    pub fn basin_size(&self, coord: &Coord) -> usize {
        let mut counted = HashSet::new();
        let mut current = vec![coord];

        while !current.is_empty() {
            let mut next = vec![];

            for coord in current.iter() {
                if !counted.contains(coord) {
                    counted.insert(coord.clone());
                }

                let height = self.points.get(coord).unwrap();

                self.adjacent_points(coord)
                    .into_iter()
                    .filter(|point| {
                        if let Some((_adj_coord, &adj_height)) = point {
                            adj_height > *height && adj_height < 9
                        } else {
                            false
                        }
                    })
                    .for_each(|point| next.push(point.unwrap().0))
            }

            current = next;
        }

        counted.len()
    }

    pub fn adjacent_points(&self, coord: &Coord) -> [Option<(&Coord, &u32)>; 4] {
        [
            self.points.get_key_value(&Coord(coord.0 - 1, coord.1)),
            self.points.get_key_value(&Coord(coord.0 + 1, coord.1)),
            self.points.get_key_value(&Coord(coord.0, coord.1 - 1)),
            self.points.get_key_value(&Coord(coord.0, coord.1 + 1)),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_thing_from_str() {
        assert_eq!(
            HeightMap::new(get_test_input::<String>())
                .points
                .get(&Coord(1, 2)),
            Some(&8)
        );
    }
}
