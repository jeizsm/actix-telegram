use super::*;

/// Use this method to get a sticker set. On success, a StickerSet object is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetStickerSet {
    /// Name of the sticker set
    pub name: String,
}