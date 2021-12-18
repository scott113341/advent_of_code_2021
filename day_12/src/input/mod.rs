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
pub fn get_test_input<ParseAs>(input: usize) -> Vec<ParseAs>
where
    ParseAs: FromStr,
    <ParseAs as FromStr>::Err: Debug,
{
    let str = match input {
        1 => include_str!("test_input_1.txt"),
        2 => include_str!("test_input_2.txt"),
        3 => include_str!("test_input_3.txt"),
        _ => panic!(),
    };

    str.trim().split("\n").map(|s| s.parse().unwrap()).collect()
}
