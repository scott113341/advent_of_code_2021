use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct Thing {}

impl FromStr for Thing {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Thing {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;

    #[test]
    fn test_thing_from_str() {
        assert_eq!(get_test_input::<Thing>()[0], Thing {});
    }
}
