use crate::types::*;

/// Use this method to set the score of the specified user in a game. On success, if the message was sent by the bot, returns the edited Message, otherwise returns True. Returns an error, if the new score is not greater than the user's current score in the chat and force is False.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "MessageOrTrue"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct SetGameScore {
    /// User identifier
    pub(crate) user_id: Integer,
    /// New score, must be non-negative
    pub(crate) score: Integer,
    /// Pass True, if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) force: Option<bool>,
    /// Pass True, if the game message should not be automatically edited to include the current scoreboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) disable_edit_message: Option<bool>,
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
