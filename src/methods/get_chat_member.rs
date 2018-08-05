use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetChatMember {
    chat_id: ChatId,
    user_id: Integer,
}