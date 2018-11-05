use types::*;

/// Use this method to forward messages of any kind. On success, the sent Message is returned.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "Message"]
#[new(vis = "pub")]
#[set(vis = "pub", optional)]
pub struct ForwardMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatIdOrUsername,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format @channelusername)
    from_chat_id: ChatIdOrUsername,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    /// Message identifier in the chat specified in from_chat_id
    message_id: Integer,
}
