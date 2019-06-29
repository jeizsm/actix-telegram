use crate::types::*;

/// Use this method to get data for high score tables. Will return the score of the specified user and several of his neighbors in a game. On success, returns an Array of GameHighScore objects.
/// 
/// This method will currently return scores for the target user, plus two of his closest neighbors on each side. Will also return the top three users if the user and his neighbors are not among them. Please note that this behavior is subject to change.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "Vec<GameHighScore>"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct GetGameHighScores {
    /// Target user id
    pub(crate) user_id: Integer,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) chat_id: Option<Integer>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) message_id: Option<Integer>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) inline_message_id: Option<String>,
}