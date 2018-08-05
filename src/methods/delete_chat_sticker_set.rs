use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteChatStickerSet {
    chat_id: ChatId,
}