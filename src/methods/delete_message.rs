use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteMessage {
    chat_id: ChatId,
    message_id: Integer,
}