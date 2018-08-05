use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct StickerSet {
    name: String,
    title: String,
    contains_masks: bool,
    stickers: Vec<Sticker>,
}