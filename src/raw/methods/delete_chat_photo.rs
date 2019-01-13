use crate::types::*;

/// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
///
/// Note: In regular groups (non-supergroups), this method will only work if the ‘All Members Are Admins’ setting is off in the target group.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "True"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct DeleteChatPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub(crate) chat_id: ChatIdOrUsername,
}
