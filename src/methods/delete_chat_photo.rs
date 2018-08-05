use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteChatPhoto {
    chat_id: ChatId,
}