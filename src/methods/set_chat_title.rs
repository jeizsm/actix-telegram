use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetChatTitle {
    chat_id: ChatId,
    title: String,
}