use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetStickerSet {
    name: String,
}