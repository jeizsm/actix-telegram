use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct KickChatMember {
    chat_id: ChatId,
    user_id: Integer,
    until_date: Option<Integer>,
}