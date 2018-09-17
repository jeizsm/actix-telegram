use types::*;

/// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
///
/// Note: In regular groups (non-supergroups), this method will only work if the ‘All Members Are Admins’ setting is off in the target group.
#[derive(Debug, Serialize, TelegramApi)]
#[return_type = "True"]
pub struct SetChatPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatIdOrUsername,
    /// New chat photo, uploaded using multipart/form-data
    pub photo: InputFile,
}