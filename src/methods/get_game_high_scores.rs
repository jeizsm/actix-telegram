use super::super::types::*;

/// 
/// This method will currently return scores for the target user, plus two of his closest neighbors on each side. Will also return the top three users if the user and his neighbors are not among them. Please note that this behavior is subject to change.
/// 
/// Use this method to get data for high score tables. Will return the score of the specified user and several of his neighbors in a game. On success, returns an Array of GameHighScore objects.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetGameHighScores {
    user_id: Integer,
    chat_id: Option<Integer>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
}