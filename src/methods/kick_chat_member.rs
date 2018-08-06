use super::super::types::*;

/// 
/// Note: In regular groups (non-supergroups), this method will only work if the ‘All Members Are Admins’ setting is off in the target group. Otherwise members may only be removed by the group's creator or by the member that added them.
/// 
/// Use this method to kick a user from a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the group on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct KickChatMember {
    chat_id: ChatId,
    user_id: Integer,
    until_date: Option<Integer>,
}