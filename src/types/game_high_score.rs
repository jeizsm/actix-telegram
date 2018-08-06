use super::*;

/// This object represents one row of the high scores table for a game.
#[derive(Serialize, Deserialize, Debug)]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: Integer,
    /// User
    pub user: User,
    /// Score
    pub score: Integer,
}