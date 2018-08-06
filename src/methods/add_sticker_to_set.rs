use super::super::types::*;

/// Use this method to add a new sticker to a set created by the bot. Returns True on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct AddStickerToSet {
    user_id: Integer,
    name: String,
    png_sticker: PngSticker,
    emojis: String,
    mask_position: Option<MaskPosition>,
}