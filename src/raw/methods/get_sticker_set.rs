use types::*;

/// Use this method to get a sticker set. On success, a StickerSet object is returned.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "StickerSet"]
#[new(vis = "pub")]
#[set(vis = "pub", optional)]
pub struct GetStickerSet {
    /// Name of the sticker set
    name: String,
}
