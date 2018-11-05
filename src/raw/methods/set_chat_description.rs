use types::*;

/// Use this method to change the description of a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "True"]
#[new(vis = "pub")]
#[set(vis = "pub", optional)]
pub struct SetChatDescription {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatIdOrUsername,
    /// New chat description, 0-255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}
