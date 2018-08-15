use types::*;

/// Use this method to unban a previously kicked user in a supergroup or channel. The user will not return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. Returns True on success.
#[derive(Serialize, Debug, TelegramApi)]
#[return_type = "True"]
pub struct UnbanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format @username)
    pub chat_id: ChatIdOrUsername,
    /// Unique identifier of the target user
    pub user_id: Integer,
}
