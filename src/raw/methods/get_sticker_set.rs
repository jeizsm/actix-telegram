use types::*;

/// Use this method to get a sticker set. On success, a StickerSet object is returned.
#[derive(Debug, Serialize, TelegramApi)]
#[return_type = "StickerSet"]
pub struct GetStickerSet {
    /// Name of the sticker set
    pub name: String,
}
