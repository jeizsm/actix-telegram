use super::super::types::*;

/// Use this method to unban a previously kicked user in a supergroup or channel. The user will not return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. Returns True on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct UnbanChatMember {
    chat_id: ChatId,
    user_id: Integer,
}