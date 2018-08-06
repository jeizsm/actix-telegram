use super::super::types::*;

/// 
/// Note: In regular groups (non-supergroups), this method will only work if the ‘All Members Are Admins’ setting is off in the target group.
/// 
/// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success. 
#[derive(Serialize, Deserialize, Debug)]
pub struct SetChatTitle {
    chat_id: ChatId,
    title: String,
}