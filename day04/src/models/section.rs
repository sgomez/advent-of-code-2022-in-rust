use itertools::Itertools;

pub struct Section {
    starts: u32,
    ends: u32,
}

impl Section {
    pub fn from_str(input: &str) -> Self {
        let (starts, ends): (u32, u32) = input
            .split("-")
            .map(|section| section.parse().unwrap())
            .collect_tuple()
            .unwrap();

        Self { starts, ends }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.starts <= other.starts && self.ends >= other.ends
            || self.starts >= other.starts && self.ends <= other.ends
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.ends >= other.starts && other.ends >= self.starts
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("2-4", "6-8" => false ; "no contains each other")]
    #[test_case("2-8", "3-7" => true ; "contains first")]
    #[test_case("6-6", "4-6" => true ; "contains second")]
    fn checks_contains(first: &str, second: &str) -> bool {
        // Arrange
        let first = Section::from_str(first);
        let second = Section::from_str(second);

        // Act
        let overlap = first.contains(&second);

        // Assert
        overlap
    }

    #[test_case("2-4", "6-8" => false ; "no overlaps each other")]
    #[test_case("5-7", "7-9" => true ; "overlaps in 7")]
    #[test_case("2-8", "3-7" => true ; "overlaps between 3-7")]
    #[test_case("6-6", "4-6" => true ; "overlaps in 6")]
    #[test_case("2-6", "4-8" => true ; "overlaps between 4-6")]
    fn checks_overlaps(first: &str, second: &str) -> bool {
        // Arrange
        let first = Section::from_str(first);
        let second = Section::from_str(second);

        // Act
        let overlap = first.overlaps(&second);

        // Assert
        overlap
    }
}
