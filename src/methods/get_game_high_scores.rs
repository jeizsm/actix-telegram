use super::super::types::*;

/// 
/// This method will currently return scores for the target user, plus two of his closest neighbors on each side. Will also return the top three users if the user and his neighbors are not among them. Please note that this behavior is subject to change.
/// 
/// Use this method to get data for high score tables. Will return the score of the specified user and several of his neighbors in a game. On success, returns an Array of GameHighScore objects.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetGameHighScores {
    /// Target user id
    pub user_id: Option<Integer>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    pub chat_id: Option<Integer>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    pub message_id: Option<Integer>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
}