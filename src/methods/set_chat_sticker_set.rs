use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetChatStickerSet {
    chat_id: ChatId,
    sticker_set_name: String,
}