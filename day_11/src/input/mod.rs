use crate::OctoGrid;

pub fn get_input() -> OctoGrid {
    let initial = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    OctoGrid::new(initial)
}

#[allow(dead_code)]
pub fn get_test_input() -> OctoGrid {
    let initial = include_str!("test_input.txt")
        .trim()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    OctoGrid::new(initial)
}
