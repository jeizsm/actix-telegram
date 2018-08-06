use super::super::types::*;

/// Use this method to forward messages of any kind. On success, the sent Message is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardMessage {
    chat_id: ChatId,
    from_chat_id: FromChatId,
    disable_notification: Option<bool>,
    message_id: Integer,
}