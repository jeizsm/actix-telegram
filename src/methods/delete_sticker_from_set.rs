use super::super::types::*;

/// Use this method to delete a sticker from a set created by the bot. Returns True on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteStickerFromSet {
    sticker: String,
}