use super::*;

/// Use this method to move a sticker in a set created by the bot to a specific position . Returns True on success.
#[derive(Serialize, Debug, TelegramApi)]
#[return_type = "True"]
pub struct SetStickerPositionInSet {
    /// File identifier of the sticker
    pub sticker: String,
    /// New sticker position in the set, zero-based
    pub position: Integer,
}
