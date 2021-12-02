use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct Command {
    pub direction: Direction,
    pub units: usize,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;

        let (direction_str, units_str) = s
            .split_once(' ')
            .ok_or(format!("Could not parse command: {}", s))?;

        let direction = match direction_str {
            "forward" => Ok(Forward),
            "down" => Ok(Down),
            "up" => Ok(Up),
            _ => Err(format!("Unknown direction: {}", direction_str)),
        }?;

        let units = units_str
            .parse()
            .map_err(|_| format!("Unknown units: {}", units_str))?;

        Ok(Command { direction, units })
    }
}

#[derive(Eq, PartialEq, Default, Debug)]
pub struct Position {
    pub horizontal: usize,
    pub depth: usize,
    pub aim: usize,
}

impl Position {
    pub fn process_command_part_1(&mut self, command: &Command) {
        use Direction::*;

        let units = command.units;

        match command.direction {
            Forward => self.horizontal += units,
            Down => self.depth += units,
            Up => self.depth -= units,
        };
    }

    pub fn process_command_part_2(&mut self, command: &Command) {
        use Direction::*;

        let units = command.units;

        match command.direction {
            Forward => {
                self.horizontal += units;
                self.depth += self.aim * units;
            }
            Down => self.aim += units,
            Up => self.aim -= units,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_test_input;
    use Direction::*;

    #[test]
    fn test_command_from_str() {
        assert_eq!(
            get_test_input::<Command>()[0],
            Command {
                direction: Forward,
                units: 5
            },
        );
    }
}
