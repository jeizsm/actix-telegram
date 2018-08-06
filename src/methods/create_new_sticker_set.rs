use super::super::types::*;

/// Use this method to create new sticker set owned by a user. The bot will be able to edit the created sticker set. Returns True on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNewStickerSet {
    user_id: Integer,
    name: String,
    title: String,
    png_sticker: PngSticker,
    emojis: String,
    contains_masks: Option<bool>,
    mask_position: Option<MaskPosition>,
}