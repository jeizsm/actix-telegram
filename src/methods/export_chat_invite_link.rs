use super::super::types::*;

/// Use this method to generate a new invite link for a chat; any previously generated link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns the new invite link as String on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct ExportChatInviteLink {
    chat_id: ChatId,
}