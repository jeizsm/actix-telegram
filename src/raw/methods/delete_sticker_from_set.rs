use types::*;

/// Use this method to delete a sticker from a set created by the bot. Returns True on success.
#[derive(Debug, Serialize, TelegramApi)]
#[return_type = "True"]
pub struct DeleteStickerFromSet {
    /// File identifier of the sticker
    pub sticker: String,
}
