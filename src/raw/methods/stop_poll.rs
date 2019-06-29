use crate::types::*;

/// Use this method to stop a poll which was sent by the bot. On success, the stopped Poll with the final results is returned.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "Poll"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct StopPoll {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub(crate) chat_id: ChatIdOrUsername,
    /// Identifier of the original message with the poll
    pub(crate) message_id: Integer,
    /// A JSON-serialized object for a new message inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_markup: Option<InlineKeyboardMarkup>,
}