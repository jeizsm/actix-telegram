use super::super::types::*;

/// Use this method to get information about a member of a chat. Returns a ChatMember object on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetChatMember {
    chat_id: ChatId,
    user_id: Integer,
}