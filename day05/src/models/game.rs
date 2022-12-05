use itertools::Itertools;

use super::{procedure::Procedure, ship::Ship};

pub struct Game {
    ship: Ship,
    procedures: Vec<Procedure>,
}

impl Game {
    pub fn from_str(input: &str) -> Self {
        let lines: Vec<&str> = input.split("\n").collect();

        let times: usize = lines[0].parse().unwrap();
        let times = times + 1;

        let stacks: Vec<&str> = lines[1..times].to_vec();

        let procedures: Vec<Procedure> = lines[times..]
            .iter()
            .map(|procedure| Procedure::from_str(procedure))
            .collect();

        let ship = Ship::from_vec(stacks);

        Self { ship, procedures }
    }

    pub fn run_procedures(&mut self) {
        self.procedures
            .iter()
            .for_each(|procedure| self.ship.run_procedure(procedure));
    }

    pub fn run_procedures_in_block(&mut self) {
        self.procedures
            .iter()
            .for_each(|procedure| self.ship.run_procedure_in_block(procedure));
    }

    pub fn get_top_stacks(&self) -> String {
        self.ship
            .stacks()
            .iter()
            .map(|stack| stack.top().unwrap())
            .join("")
    }
}
