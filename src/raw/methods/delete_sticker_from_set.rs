use crate::types::*;

/// Use this method to delete a sticker from a set created by the bot. Returns True on success.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "True"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct DeleteStickerFromSet {
    /// File identifier of the sticker
    pub(crate) sticker: String,
}