use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct LeaveChat {
    chat_id: ChatId,
}