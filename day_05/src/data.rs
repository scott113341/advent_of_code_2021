use std::str::FromStr;

#[derive(Eq, PartialEq, Hash, Debug)]
// Coord(x, y)
pub struct Coord(usize, usize);

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Coord(x.parse().unwrap(), y.parse().unwrap()))
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct VentLine {
    pub start: Coord,
    pub end: Coord,
}

impl FromStr for VentLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s.split_once(" -> ").unwrap();

        Ok(VentLine {
            start: start_str.parse().unwrap(),
            end: end_str.parse().unwrap(),
        })
    }
}

impl VentLine {
    pub fn coords(&self) -> Vec<Coord> {
        let mut coords = vec![];

        let min_x = self.start.0.min(self.end.0);
        let max_x = self.start.0.max(self.end.0);
        let min_y = self.start.1.min(self.end.1);
        let max_y = self.start.1.max(self.end.1);

        if self.is_diagonal() {
            let x_increases = self.start.0 < self.end.0;
            let y_increases = self.start.1 < self.end.1;
            let n_coords = max_x - min_x;

            for i in 0..=n_coords {
                match (x_increases, y_increases) {
                    (true, true) => coords.push(Coord(self.start.0 + i, self.start.1 + i)),
                    (true, false) => coords.push(Coord(self.start.0 + i, self.start.1 - i)),
                    (false, true) => coords.push(Coord(self.start.0 - i, self.start.1 + i)),
                    (false, false) => coords.push(Coord(self.start.0 - i, self.start.1 - i)),
                }
            }
        } else {
            if self.start.0 == self.end.0 {
                // Constant x, non-diagonal
                for y in min_y..=max_y {
                    coords.push(Coord(self.start.0, y));
                }
            } else {
                // Constant y, non-diagonal
                for x in min_x..=max_x {
                    coords.push(Coord(x, self.start.1));
                }
            }
        }

        coords
    }

    pub fn is_diagonal(&self) -> bool {
        self.start.0 != self.end.0 && self.start.1 != self.end.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_coord_and_vent_line_from_str() {
        assert_eq!(
            get_test_input::<VentLine>()[0],
            VentLine {
                start: Coord(0, 9),
                end: Coord(5, 9)
            }
        );
    }

    #[test]
    fn test_coord_vent_line_coords() {
        assert_eq!(
            "1,1 -> 1,3".parse::<VentLine>().unwrap().coords(),
            vec![Coord(1, 1), Coord(1, 2), Coord(1, 3)]
        );

        assert_eq!(
            "9,7 -> 7,7".parse::<VentLine>().unwrap().coords(),
            vec![Coord(7, 7), Coord(8, 7), Coord(9, 7)]
        );

        assert_eq!(
            "1,1 -> 3,3".parse::<VentLine>().unwrap().coords(),
            vec![Coord(1, 1), Coord(2, 2), Coord(3, 3)]
        );

        assert_eq!(
            "9,7 -> 7,9".parse::<VentLine>().unwrap().coords(),
            vec![Coord(9, 7), Coord(8, 8), Coord(7, 9)]
        );
    }
}
