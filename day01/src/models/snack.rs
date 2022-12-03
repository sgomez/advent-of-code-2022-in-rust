#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct Snack {
    calories: u32,
}

impl Snack {
    pub fn new(calories: u32) -> Self {
        Self { calories }
    }

    pub fn calories(&self) -> u32 {
        self.calories
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Snack;

    #[test]
    fn snacks_has_calories() {
        let snack = Snack::new(1000);
        assert_eq!(snack.calories, 1000);
    }
}
