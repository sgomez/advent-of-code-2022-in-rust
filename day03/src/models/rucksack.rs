use super::{compartment::Compartment, item::Item};

#[derive(Debug, Clone)]
pub struct Rucksack {
    compartment_left: Compartment,
    compartment_right: Compartment,
}

impl Rucksack {
    pub fn build_from_str(input: &str) -> Self {
        let (left, right) = input.split_at(input.len() / 2);

        Rucksack {
            compartment_left: Compartment::build_from_str(left),
            compartment_right: Compartment::build_from_str(right),
        }
    }

    pub fn find_common_item(self) -> Item {
        self.compartment_left
            .items()
            .iter()
            .fold(Item::new('!'), |_prev, current| {
                if self.compartment_right.items().contains(&current) {
                    *current
                } else {
                    _prev
                }
            })
    }

    pub fn items(self) -> Vec<Item> {
        let items_left = self.compartment_left.items().clone();
        let items_right = self.compartment_right.items().clone();

        [items_left, items_right].concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_from_string() {
        // Arrange
        let input = "abcdefgh";
        // Act
        let ruckstack = Rucksack::build_from_str(input);
        // Assert
        assert_eq!(
            ruckstack.compartment_left,
            Compartment::build_from_str("abcd")
        );
        assert_eq!(
            ruckstack.compartment_right,
            Compartment::build_from_str("efgh")
        );
    }

    #[test]
    fn find_current_item() {
        // Arrange
        let input = "abcddfgh";
        // Act
        let ruckstack = Rucksack::build_from_str(input);
        // Assert
        assert_eq!(ruckstack.find_common_item(), Item::new('d'))
    }

    #[test]
    fn get_items() {
        // Arrange
        let input = "az";
        // Act
        let ruckstack = Rucksack::build_from_str(input);
        // Assert
        assert_eq!(ruckstack.items(), [Item::new('a'), Item::new('z')])
    }
}
