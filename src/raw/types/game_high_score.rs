use crate::types::*;

/// This object represents one row of the high scores table for a game.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct GameHighScore {
    /// Position in high score table for the game
    position: Integer,
    /// User
    user: User,
    /// Score
    score: Integer,
}
