use std::{
    collections::{HashSet, VecDeque},
    ops::Deref,
};

pub struct Datastream {
    buffer: Vec<char>,
    trace_size: usize,
}

impl Datastream {
    pub fn from_str(input: &str, trace_size: usize) -> Self {
        let buffer = input.chars().collect();

        Self { buffer, trace_size }
    }

    pub fn start_of_message(&self) -> usize {
        let from = self.trace_size - 1;

        let mut sequence: VecDeque<&char> = self.buffer[..from].iter().collect();

        for (index, code) in self.buffer[from..].iter().enumerate() {
            sequence.push_back(code);

            if self.validate_sequence(&sequence) {
                return index + self.trace_size;
            }

            sequence.pop_front();
        }

        panic!("Not found");
    }

    fn validate_sequence(&self, sequence: &VecDeque<&char>) -> bool {
        let set: HashSet<&char> = sequence.iter().map(Deref::deref).collect();

        set.len() == self.trace_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    // #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4 => 7; "1st test case with trace size of 4")]
    // #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 4 => 5; "2nd test case with trace size of 4")]
    // #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 4 => 6; "3rd test case with trace size of 4")]
    // #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4 => 10; "4th test case with trace size of 4")]
    // #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4 => 11; "5th test case with trace size of 4")]
    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14 => 19; "1st test case with trace size of 14")]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 14 => 23; "2nd test case with trace size of 14")]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 14 => 23; "3rd test case with trace size of 14")]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14 => 29; "4th test case with trace size of 14")]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14 => 26; "5th test case with trace size of 14")]
    pub fn find_signal(input: &str, trace_size: usize) -> usize {
        // Arrange
        let datastream = Datastream::from_str(input, trace_size);

        // Act
        let marker_index = datastream.start_of_message();

        // Assert
        marker_index
    }
}
