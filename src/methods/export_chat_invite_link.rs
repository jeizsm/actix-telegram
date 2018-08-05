use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportChatInviteLink {
    chat_id: ChatId,
}