use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PinChatMessage {
    chat_id: ChatId,
    message_id: Integer,
    disable_notification: Option<bool>,
}