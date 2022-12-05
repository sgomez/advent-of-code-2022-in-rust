use derive_getters::Getters;

use super::procedure::Procedure;
use super::stack::Stack;

#[derive(Getters)]
pub struct Ship {
    stacks: Vec<Stack>,
}

impl Ship {
    pub fn from_vec(input: Vec<&str>) -> Self {
        let stacks = input.iter().map(|stack| Stack::from_str(stack)).collect();

        Ship { stacks }
    }

    pub fn run_procedure(&mut self, procedure: &Procedure) {
        let times = *procedure.times();
        let from = *procedure.from() - 1;
        let to = *procedure.to() - 1;

        for _ in 0..times {
            let from_stack = &mut self.stacks[from];
            let moved_cargo = from_stack.pop();

            let to_stack = &mut self.stacks[to];
            match moved_cargo {
                Some(cargo) => to_stack.push(cargo),
                None => panic!("No cargo!!!"),
            }
        }
    }

    pub fn run_procedure_in_block(&mut self, procedure: &Procedure) {
        let times = *procedure.times();
        let from = *procedure.from() - 1;
        let to = *procedure.to() - 1;

        let from_stack = &mut self.stacks[from];
        let moved_cargo = from_stack.npop(times);

        let to_stack = &mut self.stacks[to];
        to_stack.npush(moved_cargo);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn run_procedure_one_by_one() {
        // Arrange
        let mut ship = Ship::from_vec(["ABC", "D", "EF"].to_vec());
        let procedure = Procedure::from_str("move 2 from 1 to 2");

        // Act
        ship.run_procedure(&procedure);

        // Assert
        assert_eq!(ship.stacks()[0].top(), Some(&'A'));
        assert_eq!(ship.stacks()[1].top(), Some(&'B'));
        assert_eq!(ship.stacks()[2].top(), Some(&'F'));
    }

    #[test]
    pub fn run_procedure_in_block() {
        // Arrange
        let mut ship = Ship::from_vec(["ABC", "D", "EF"].to_vec());
        let procedure = Procedure::from_str("move 2 from 1 to 2");

        // Act
        ship.run_procedure_in_block(&procedure);

        // Assert
        assert_eq!(ship.stacks()[0].top(), Some(&'A'));
        assert_eq!(ship.stacks()[1].top(), Some(&'C'));
        assert_eq!(ship.stacks()[2].top(), Some(&'F'));
    }
}
