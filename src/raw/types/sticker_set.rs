use crate::types::*;

/// This object represents a sticker set.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct StickerSet {
    /// Sticker set name
    pub(crate) name: String,
    /// Sticker set title
    pub(crate) title: String,
    /// True, if the sticker set contains masks
    pub(crate) contains_masks: bool,
    /// List of all set stickers
    pub(crate) stickers: Vec<Sticker>,
}
