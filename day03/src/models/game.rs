use super::{group::Group, item::Item, rucksack::Rucksack};

#[derive(Debug, Clone)]
pub struct Game {
    rucksacks: Vec<Rucksack>,
}

impl Game {
    pub fn build_from_str(input: &str) -> Self {
        let rucksacks = input
            .split("\n")
            .map(|input| Rucksack::build_from_str(input))
            .collect();
        Self { rucksacks }
    }

    pub fn find_common_items(self) -> Vec<Item> {
        self.rucksacks
            .into_iter()
            .map(|rucksack| rucksack.find_common_item())
            .collect()
    }

    pub fn find_common_items_by_group(self) -> Vec<Item> {
        self.rucksacks
            .chunks(3)
            .map(|chunk| Group::new(chunk.to_vec()))
            .map(|group| group.find_common_item())
            .collect()
    }
}
