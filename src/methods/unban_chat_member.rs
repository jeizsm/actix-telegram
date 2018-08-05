use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct UnbanChatMember {
    chat_id: ChatId,
    user_id: Integer,
}