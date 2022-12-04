#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Item {
    pub value: char,
}

impl Item {
    pub fn new(value: char) -> Self {
        Self { value }
    }

    pub fn priority(self) -> u32 {
        match self.value {
            'a'..='z' => self.value as u32 - 'a' as u32 + 1,
            'A'..='Z' => self.value as u32 - 'A' as u32 + 27,
            _ => panic!("Invalid value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case('a' => 1 ; "value of item lowercase a")]
    #[test_case('z' => 26 ; "value of item lowercase z")]
    #[test_case('A' => 27 ; "value of item uppercase A")]
    fn get_value_of_item(item: char) -> u32 {
        // Arrange
        // Act
        let item = Item::new(item);

        // Assert
        item.priority()
    }
}
