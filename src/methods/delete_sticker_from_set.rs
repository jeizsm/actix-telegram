use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteStickerFromSet {
    sticker: String,
}