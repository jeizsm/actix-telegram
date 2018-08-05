use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct UnpinChatMessage {
    chat_id: ChatId,
}