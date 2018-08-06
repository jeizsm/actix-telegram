use super::super::types::*;

/// Use this method to move a sticker in a set created by the bot to a specific position . Returns True on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct SetStickerPositionInSet {
    /// File identifier of the sticker
    pub sticker: Option<String>,
    /// New sticker position in the set, zero-based
    pub position: Option<Integer>,
}