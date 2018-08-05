use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetChatDescription {
    chat_id: ChatId,
    description: Option<String>,
}