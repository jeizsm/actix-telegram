use types::*;

/// Use this method to get the number of members in a chat. Returns Int on success.
#[derive(Debug, Serialize, TelegramApi)]
#[return_type = "Integer"]
pub struct GetChatMembersCount {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: ChatIdOrUsername,
}
