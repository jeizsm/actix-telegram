use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetChatAdministrators {
    chat_id: ChatId,
}