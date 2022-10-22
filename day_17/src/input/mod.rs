use std::fmt::Debug;
use std::str::FromStr;

pub fn get_input<ParseAs>() -> ParseAs
where
    ParseAs: FromStr,
    <ParseAs as FromStr>::Err: Debug,
{
    include_str!("input.txt").trim().parse().unwrap()
}

#[allow(dead_code)]
pub fn get_test_input<ParseAs>() -> ParseAs
where
    ParseAs: FromStr,
    <ParseAs as FromStr>::Err: Debug,
{
    include_str!("test_input.txt").trim().parse().unwrap()
}
