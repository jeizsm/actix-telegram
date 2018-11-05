use types::*;

/// This object represents a sticker set.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct StickerSet {
    /// Sticker set name
    name: String,
    /// Sticker set title
    title: String,
    /// True, if the sticker set contains masks
    contains_masks: bool,
    /// List of all set stickers
    stickers: Vec<Sticker>,
}
