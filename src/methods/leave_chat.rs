use super::*;

/// Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct LeaveChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: ChatId,
}
