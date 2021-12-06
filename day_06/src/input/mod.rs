use std::fmt::Debug;
use std::str::FromStr;

pub fn get_input<ParseAs>() -> Vec<ParseAs>
where
    ParseAs: FromStr,
    <ParseAs as FromStr>::Err: Debug,
{
    include_str!("input.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn get_test_input<ParseAs>() -> Vec<ParseAs>
where
    ParseAs: FromStr,
    <ParseAs as FromStr>::Err: Debug,
{
    include_str!("test_input.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}
