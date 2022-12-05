use derive_getters::Getters;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Getters)]
pub struct Procedure {
    pub times: usize,
    pub from: usize,
    pub to: usize,
}

impl Procedure {
    pub fn from_str(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^move (?P<times>\d+) from (?P<from>\d+) to (?P<to>\d+)$").unwrap();
        }

        let captures = RE.captures(input).unwrap();
        let times: usize = captures.name("times").unwrap().as_str().parse().unwrap();
        let from: usize = captures.name("from").unwrap().as_str().parse().unwrap();
        let to: usize = captures.name("to").unwrap().as_str().parse().unwrap();

        Self { times, from, to }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_from_string() {
        // Arrange
        let input = "move 4 from 1 to 2";

        // Act
        let procedure = Procedure::from_str(input);

        // Assert
        assert_eq!(procedure.times, 4);
        assert_eq!(procedure.from, 1);
        assert_eq!(procedure.to, 2);
    }
}
