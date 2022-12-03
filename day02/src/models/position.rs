use crate::models::score::Score;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Position {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Position {
    pub fn build_from_str(input: &str) -> Self {
        match input {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Input not valid"),
        }
    }

    pub fn build_to_score_against(position: Position, score: Score) -> Self {
        match score {
            Score::Win => position.lose_against(),
            Score::Draw => position,
            Score::Loss => position.win_against(),
        }
    }

    pub fn play(&self, other: Self) -> u32 {
        let points_per_shape = *self as u32;
        let points_per_play = (match self {
            _ if self.win_against() == other => Score::Win,
            _ if self.lose_against() == other => Score::Loss,
            _ => Score::Draw,
        }) as u32;

        points_per_play + points_per_shape
    }

    pub fn lose_against(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn win_against(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_win_scissors() {
        // Arrange
        let player1 = Position::Rock;
        let player2 = Position::Scissors;
        // Act
        let scores = player1.play(player2);

        // Assert
        assert_eq!(scores, Score::Win as u32 + Position::Rock as u32)
    }

    #[test]
    fn rock_draw_rock() {
        // Arrange
        let player1 = Position::build_from_str("A");
        let player2 = Position::build_from_str("X");
        // Act
        let scores = player1.play(player2);

        // Assert
        assert_eq!(scores, Score::Draw as u32 + Position::Rock as u32)
    }

    #[test]
    fn rock_lose_paper() {
        // Arrange
        let player1 = Position::Rock;
        let player2 = Position::Paper;
        // Act
        let scores = player1.play(player2);

        // Assert
        assert_eq!(scores, Score::Loss as u32 + Position::Rock as u32)
    }

    #[test]
    fn rock_lose_against_paper() {
        // Arrange
        let player1 = Position::Rock;
        let player2 = player1.lose_against();
        // Act
        let scores = player1.play(player2);

        // Assert
        assert_eq!(scores, Score::Loss as u32 + Position::Rock as u32)
    }
}
