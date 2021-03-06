use crate::types::*;

/// Use this method to get up to date information about the chat (current name of the user for one-on-one conversations, current username of a user, group or channel, etc.). Returns a Chat object on success.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "Chat"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct GetChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub(crate) chat_id: ChatIdOrUsername,
}