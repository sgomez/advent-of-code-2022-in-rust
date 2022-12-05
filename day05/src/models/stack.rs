pub struct Stack {
    crates: Vec<char>,
}

impl Stack {
    pub fn from_str(input: &str) -> Self {
        let crates = input.chars().collect();

        Self { crates }
    }

    pub fn push(&mut self, input: char) {
        self.crates.push(input);
    }

    pub fn npush(&mut self, input: Vec<char>) {
        self.crates.extend(input)
    }

    pub fn pop(&mut self) -> Option<char> {
        self.crates.pop()
    }

    pub fn npop(&mut self, size: usize) -> Vec<char> {
        let at = self.crates.len() - size;

        self.crates.split_off(at)
    }

    pub fn top(&self) -> Option<&char> {
        self.crates.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn can_push_and_pop() {
        // Arrange
        let mut stack = Stack::from_str("AB");
        // Act
        stack.push('C');
        let result = stack.pop();

        // Assert
        let expected_result = Some('C');
        assert_eq!(result, expected_result);
    }

    #[test]
    pub fn get_get_top() {
        // Arrange
        let stack = Stack::from_str("ABC");

        // Act
        let result = stack.top();

        // Assert
        let expected_result = Some(&'C');
        assert_eq!(result, expected_result);
    }

    #[test]
    pub fn can_push_and_pop_n_elements() {
        // Arrange
        let mut stack = Stack::from_str("ABC");
        let mut stack2 = Stack::from_str("D");

        // Act
        let result = stack.npop(2);
        stack2.npush(result);

        // Assert
        assert_eq!(stack.top(), Some(&'A'));
        assert_eq!(stack2.top(), Some(&'C'));
    }
}
