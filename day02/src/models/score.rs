#[derive(Debug, PartialEq)]
pub enum Score {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl Score {
    pub fn build_from_str(input: &str) -> Self {
        match input {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Input not valid"),
        }
    }
}
