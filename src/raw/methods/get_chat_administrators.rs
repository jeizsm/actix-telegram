use crate::types::*;

/// Use this method to get a list of administrators in a chat. On success, returns an Array of ChatMember objects that contains information about all chat administrators except other bots. If the chat is a group or a supergroup and no administrators were appointed, only the creator will be returned.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "Vec<ChatMember>"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct GetChatAdministrators {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub(crate) chat_id: ChatIdOrUsername,
}