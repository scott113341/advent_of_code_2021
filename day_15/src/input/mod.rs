use std::fmt::Debug;
use std::str::FromStr;

pub fn get_input<ParseAs>() -> Vec<ParseAs>
where
    ParseAs: FromStr,
    <ParseAs as FromStr>::Err: Debug,
{
    include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn get_test_input<ParseAs>(which_one: usize) -> Vec<ParseAs>
where
    ParseAs: FromStr,
    <ParseAs as FromStr>::Err: Debug,
{
    let str = match which_one {
        1 => include_str!("test_input_1.txt"),
        2 => include_str!("test_input_2.txt"),
        _ => panic!("Wut"),
    };

    str.trim().split("\n").map(|s| s.parse().unwrap()).collect()
}
