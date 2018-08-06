use super::*;

/// This object represents one row of the high scores table for a game.
#[derive(Serialize, Deserialize, Debug)]
pub struct GameHighScore {
    position: Integer,
    user: User,
    score: Integer,
}