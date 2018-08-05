use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetChatMembersCount {
    chat_id: ChatId,
}