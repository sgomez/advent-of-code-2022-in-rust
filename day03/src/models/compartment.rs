use super::item::Item;

#[derive(Debug, PartialEq, Clone)]
pub struct Compartment {
    items: Vec<Item>,
}

impl Compartment {
    pub fn build_from_str(input: &str) -> Self {
        let items = input.chars().map(|item| Item::new(item)).collect();

        Compartment { items }
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_from_string() {
        // Arrange
        // Act
        let compartment = Compartment::build_from_str("abcd");
        // Assert

        assert_eq!(compartment.items.len(), 4);
    }
}
