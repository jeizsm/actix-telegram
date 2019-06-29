use crate::types::*;

/// Use this method to get the number of members in a chat. Returns Int on success.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "Integer"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct GetChatMembersCount {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub(crate) chat_id: ChatIdOrUsername,
}