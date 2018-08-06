use super::super::types::*;

/// Use this method to get a sticker set. On success, a StickerSet object is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetStickerSet {
    name: String,
}