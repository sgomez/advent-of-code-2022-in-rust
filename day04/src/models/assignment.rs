use super::section::Section;

pub struct Assignment {
    sections: (Section, Section),
}

impl Assignment {
    pub fn from_str(input: &str) -> Self {
        let sections: Vec<&str> = input.split(",").collect();

        Self {
            sections: (
                Section::from_str(sections[0]),
                Section::from_str(sections[1]),
            ),
        }
    }

    pub fn contains(&self) -> bool {
        self.sections.0.contains(&self.sections.1)
    }

    pub fn overlaps(&self) -> bool {
        self.sections.0.overlaps(&self.sections.1)
    }
}
