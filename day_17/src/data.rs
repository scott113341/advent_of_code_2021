use regex::Regex;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub type TargetRange = RangeInclusive<isize>;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct XY {
    pub x: isize,
    pub y: isize,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ProbeAndTarget {
    pub step: usize,
    pub target_x: TargetRange,
    pub target_y: TargetRange,
    pub velocity: XY,
    pub position: XY,
}

impl FromStr for ProbeAndTarget {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = Regex::new(
            r"^target area: x=(?P<x1>[\d-]+)..(?P<x2>[\d-]+), y=(?P<y1>[\d-]+)..(?P<y2>[\d-]+)$",
        )
        .unwrap()
        .captures(s)
        .unwrap();

        let target_x = TargetRange::new(c["x1"].parse().unwrap(), c["x2"].parse().unwrap());
        let target_y = TargetRange::new(c["y1"].parse().unwrap(), c["y2"].parse().unwrap());
        let velocity = XY { x: 0, y: 0 };

        Ok(ProbeAndTarget::new(target_x, target_y, velocity))
    }
}

impl ProbeAndTarget {
    pub fn new(target_x: TargetRange, target_y: TargetRange, velocity: XY) -> ProbeAndTarget {
        ProbeAndTarget {
            step: 0,
            target_x,
            target_y,
            velocity,
            position: XY { x: 0, y: 0 },
        }
    }

    // The probe's position increases by its velocity. The probe's x velocity changes by 1
    // toward the value 0. The probe's y velocity decreases by 1.
    pub fn step(&mut self) {
        let v = &self.velocity;

        self.step += 1;
        self.position.x += v.x;
        self.position.y += v.y;

        let vx = if v.x == 0 {
            0
        } else if v.x > 0 {
            v.x - 1
        } else {
            v.x + 1
        };
        self.velocity = XY { x: vx, y: v.y - 1 };
    }

    pub fn in_target_area(&self) -> bool {
        self.target_x.contains(&self.position.x) && self.target_y.contains(&self.position.y)
    }

    pub fn confirmed_will_never_hit_target(&self) -> bool {
        if self.in_target_area() {
            return false;
        }

        if self.velocity.x == 0 {
            let min = self.target_y.start().min(self.target_y.end());
            self.position.y < *min
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_from_str() {
        assert_eq!(
            get_test_input::<ProbeAndTarget>(),
            ProbeAndTarget {
                step: 0,
                target_x: TargetRange::new(20, 30),
                target_y: TargetRange::new(-10, -5),
                velocity: XY { x: 0, y: 0 },
                position: XY { x: 0, y: 0 },
            }
        );
    }

    #[test]
    fn test_new() {
        let pat = ProbeAndTarget::new(20..=30, -10..=-5, XY { x: 7, y: 2 });

        assert_eq!(
            pat,
            ProbeAndTarget {
                step: 0,
                target_x: 20..=30,
                target_y: -10..=-5,
                position: XY { x: 0, y: 0 },
                velocity: XY { x: 7, y: 2 },
            }
        );
    }

    #[test]
    fn test_in_target_area() {
        let mut pat = ProbeAndTarget::new(20..=30, -10..=-5, XY { x: 7, y: 2 });
        for _ in 0..7 {
            assert_eq!(pat.in_target_area(), false);
            pat.step();
        }
        assert_eq!(pat.in_target_area(), true);
    }
}
