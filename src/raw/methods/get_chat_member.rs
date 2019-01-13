use crate::types::*;

/// Use this method to get information about a member of a chat. Returns a ChatMember object on success.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "ChatMember"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct GetChatMember {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub(crate) chat_id: ChatIdOrUsername,
    /// Unique identifier of the target user
    pub(crate) user_id: Integer,
}
