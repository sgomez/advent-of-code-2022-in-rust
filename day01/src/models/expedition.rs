use super::Elf;

#[derive(PartialEq, Debug)]
pub struct Expedition {
    elfs: Vec<Elf>,
}

impl Expedition {
    pub fn new(elfs: Vec<Elf>) -> Self {
        Self { elfs }
    }

    pub fn build_from_str(input: &str) -> Self {
        let elfs = input
            .split("\n\n")
            .map(|elf| Elf::build_from_str(elf))
            .collect();

        Expedition { elfs }
    }

    pub fn total_calories_from_n_biggest(&self, n: usize) -> u32 {
        let mut ordered_elfs = self.elfs.clone();
        ordered_elfs.sort_unstable_by(|a, b| b.cmp(a));
        ordered_elfs
            .iter()
            .take(n)
            .fold(0, |accum, elf| accum + elf.total_calories())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{Elf, Expedition, Snack};

    #[test]
    fn expedition_has_elfs() {
        // Arrange
        let elf1 = Elf::new(vec![Snack::new(1000), Snack::new(2000)]);
        let elf2 = Elf::new(vec![Snack::new(2000), Snack::new(500), Snack::new(100)]);
        let elf3 = Elf::new(vec![Snack::new(1000)]);

        // Act
        let expedition = Expedition::new(vec![elf1, elf2, elf3]);

        // Assert
        assert_eq!(expedition.elfs.len(), 3)
    }

    #[test]
    fn expedition_creation_from_string() {
        // Arrange
        let elf1 = Elf::new(vec![Snack::new(1000), Snack::new(2000)]);
        let elf2 = Elf::new(vec![Snack::new(2000), Snack::new(500), Snack::new(100)]);
        let elf3 = Elf::new(vec![Snack::new(1000)]);
        let expected_expedition = Expedition::new(vec![elf1, elf2, elf3]);

        // Act
        let expedition = Expedition::build_from_str("1000\n2000\n\n2000\n500\n100\n\n1000");

        // Assert
        assert_eq!(expected_expedition, expedition)
    }

    #[test]
    fn expedition_gets_most_n_calorics_inventories() {
        // Arrange
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let expedition = Expedition::build_from_str(input);

        // Act
        let calories = expedition.total_calories_from_n_biggest(3);

        // Assert
        assert_eq!(calories, 45000)
    }
}
