use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddStickerToSet {
    user_id: Integer,
    name: String,
    png_sticker: PngSticker,
    emojis: String,
    mask_position: Option<MaskPosition>,
}