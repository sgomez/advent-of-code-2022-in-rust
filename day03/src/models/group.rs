use std::collections::HashMap;

use super::{item::Item, rucksack::Rucksack};

#[derive(Debug)]
pub struct Group {
    rucksacks: Vec<Rucksack>,
}

impl Group {
    pub fn new(rucksacks: Vec<Rucksack>) -> Self {
        Self { rucksacks }
    }

    pub fn find_common_item(self) -> Item {
        let mut frequency = HashMap::new();

        for (rucksack_index, rucksack) in self.rucksacks.into_iter().enumerate() {
            for (_, item) in rucksack.clone().items().into_iter().enumerate() {
                let value = frequency.entry(item).or_insert(0);
                *value |= 1 << rucksack_index;
            }
        }

        frequency
            .into_iter()
            .find(|&x| x.1 == 7)
            .expect("Not found")
            .0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn find_common_item() {
        // Arrange
        let rucksacks = vec![
            Rucksack::build_from_str("vJrwpWtwJgWrhcsFMMfFFhFp"),
            Rucksack::build_from_str("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            Rucksack::build_from_str("PmmdzqPrVvPwwTWBwg"),
        ];
        let group = Group::new(rucksacks);
        // Act
        let common_item = group.find_common_item();

        // Assert
        assert_eq!(common_item, Item::new('r'))
    }
}
