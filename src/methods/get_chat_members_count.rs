use super::super::types::*;

/// Use this method to get the number of members in a chat. Returns Int on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetChatMembersCount {
    chat_id: ChatId,
}