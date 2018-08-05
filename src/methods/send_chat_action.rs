use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendChatAction {
    chat_id: ChatId,
    action: String,
}