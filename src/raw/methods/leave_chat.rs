use crate::types::*;

/// Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "True"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct LeaveChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub(crate) chat_id: ChatIdOrUsername,
}
