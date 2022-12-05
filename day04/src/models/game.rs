use super::assignment::Assignment;

pub struct Game {
    assignments: Vec<Assignment>,
}

impl Game {
    pub fn from_str(input: &str) -> Self {
        let assignments: Vec<Assignment> = input
            .split("\n")
            .map(|assignnment| Assignment::from_str(assignnment))
            .collect();

        Game { assignments }
    }

    pub fn count_contains(&self) -> u32 {
        self.assignments
            .iter()
            .map(|assigment| assigment.contains())
            .filter(|result| *result)
            .collect::<Vec<bool>>()
            .len() as u32
    }

    pub fn count_overlaps(&self) -> u32 {
        self.assignments
            .iter()
            .map(|assigment| assigment.overlaps())
            .filter(|result| *result)
            .collect::<Vec<bool>>()
            .len() as u32
    }
}
