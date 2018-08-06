use super::*;

/// This object represents a sticker set.
#[derive(Serialize, Deserialize, Debug)]
pub struct StickerSet {
    name: String,
    title: String,
    contains_masks: bool,
    stickers: Vec<Sticker>,
}