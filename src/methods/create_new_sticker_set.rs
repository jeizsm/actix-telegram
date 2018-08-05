use super::super::types::*;

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