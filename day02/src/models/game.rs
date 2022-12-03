use super::{position::Position, score::Score};

pub struct Game {
    scores: u32,
}

impl Game {
    pub fn new() -> Self {
        return Self { scores: 0 };
    }

    pub fn play_strategy_one_from_str(&mut self, input: &str) {
        let players: Vec<Position> = input
            .split_whitespace()
            .map(|x| Position::build_from_str(x))
            .collect();
        let (elf, me) = (players[1], players[0]);

        self.scores += elf.play(me);
    }

    pub fn play_strategy_two_from_str(&mut self, input: &str) {
        let inputs: Vec<&str> = input.split_whitespace().collect();
        let elf = Position::build_from_str(inputs[0]);
        let my_strategy = Score::build_from_str(inputs[1]);
        let me = Position::build_to_score_against(elf, my_strategy);

        self.scores += me.play(elf);
    }

    pub fn scores(&self) -> u32 {
        self.scores
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_score_strategy_one() {
        // Arrange
        let mut game = Game::new();

        // Act
        game.play_strategy_one_from_str("A Y");
        game.play_strategy_one_from_str("B X");
        game.play_strategy_one_from_str("C Z");

        // Assert
        assert_eq!(game.scores(), 15);
    }

    #[test]
    fn calculate_score_strategy_two() {
        // Arrange
        let mut game = Game::new();

        // Act
        game.play_strategy_two_from_str("A Y");
        game.play_strategy_two_from_str("B X");
        game.play_strategy_two_from_str("C Z");

        // Assert
        assert_eq!(game.scores(), 12);
    }
}
