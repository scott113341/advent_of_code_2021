use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct Coord {
    pub y: isize,
    pub x: isize,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Fold {
    FoldX(isize),
    FoldY(isize),
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Fold::*;

        let (_, fold) = s.split_once("fold along ").ok_or(())?;
        let (axis, dist) = fold.split_once("=").unwrap();
        let dist = dist.parse().unwrap();

        match axis {
            "x" => Ok(FoldX(dist)),
            "y" => Ok(FoldY(dist)),
            _ => Err(()),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Paper {
    pub dots: BTreeSet<Coord>,
    pub folds: Vec<Fold>,
}

impl Paper {
    pub fn new(lines: &Vec<String>) -> Paper {
        let dots = lines
            .iter()
            .filter_map(|l| {
                let xy = l.split_once(',')?;
                let (x, y) = xy;
                Some(Coord {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                })
            })
            .collect();

        let folds = lines.iter().filter_map(|l| l.parse().ok()).collect();

        Paper { dots, folds }
    }

    pub fn fold(&mut self) {
        let fold = self.folds.remove(0);
        let mut new_dots = BTreeSet::new();

        for dot in self.dots.iter() {
            let mut dot = dot.clone();

            match fold {
                Fold::FoldX(x) => {
                    if dot.x > x {
                        dot.x -= (dot.x - x) * 2;
                    }
                }
                Fold::FoldY(y) => {
                    if dot.y > y {
                        dot.y -= (dot.y - y) * 2;
                    }
                }
            }

            new_dots.insert(dot);
        }

        self.dots = new_dots;
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x_max = self.dots.iter().map(|c| c.x).max().unwrap();
        let y_max = self.dots.iter().map(|c| c.y).max().unwrap();
        let mut string = String::new();

        for y in 0..=y_max {
            for x in 0..=x_max {
                if self.dots.contains(&Coord { x, y }) {
                    string.push('#');
                } else {
                    string.push(' ');
                }
            }
            string.push('\n');
        }

        f.write_str(&string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_paper_fold() {
        let mut paper = Paper::new(&get_test_input());
        assert_eq!(paper.dots.len(), 18);
        paper.fold();
        assert_eq!(paper.dots.len(), 17);
        paper.fold();
        assert_eq!(paper.dots.len(), 16);
    }
}
