use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardMessage {
    chat_id: ChatId,
    from_chat_id: FromChatId,
    disable_notification: Option<bool>,
    message_id: Integer,
}