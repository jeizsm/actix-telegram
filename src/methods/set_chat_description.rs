use super::super::types::*;

/// Use this method to change the description of a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success. 
#[derive(Serialize, Deserialize, Debug)]
pub struct SetChatDescription {
    chat_id: ChatId,
    description: Option<String>,
}