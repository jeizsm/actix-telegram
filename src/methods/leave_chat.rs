use super::super::types::*;

/// Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct LeaveChat {
    chat_id: ChatId,
}