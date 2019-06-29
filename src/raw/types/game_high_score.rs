use crate::types::*;

/// This object represents one row of the high scores table for a game.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub(crate) position: Integer,
    /// User
    pub(crate) user: User,
    /// Score
    pub(crate) score: Integer,
}