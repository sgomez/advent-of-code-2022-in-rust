use super::Snack;

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Elf {
    snacks: Vec<Snack>,
}

impl Elf {
    pub fn new(snacks: Vec<Snack>) -> Self {
        Self { snacks }
    }

    pub fn build_from_str(input: &str) -> Self {
        let snacks = input
            .split("\n")
            .map(|calories| Snack::new(calories.parse::<u32>().unwrap()))
            .collect();

        Self { snacks }
    }

    pub fn total_calories(&self) -> u32 {
        self.snacks.iter().map(|snack| snack.calories()).sum()
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_calories().cmp(&other.total_calories())
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{Elf, Snack};

    #[test]
    fn elf_has_snacks() {
        // Arrange
        // Act
        let elf = Elf::new(vec![Snack::new(1000), Snack::new(2000)]);

        // Assert
        assert_eq!(elf.snacks.len(), 2);
    }

    #[test]
    fn elf_creation_from_string() {
        // Arrange
        let expected_elf = Elf::new(vec![Snack::new(1000), Snack::new(2000)]);
        let input = "1000\n2000";

        // Act
        let elf = Elf::build_from_str(input);

        // Assert
        assert_eq!(expected_elf, elf);
    }

    #[test]
    fn elf_count_calories() {
        // Arrange
        // Act
        let elf = Elf::new(vec![Snack::new(1000), Snack::new(2000)]);

        // Assert
        assert_eq!(elf.total_calories(), 3000);
    }
}
