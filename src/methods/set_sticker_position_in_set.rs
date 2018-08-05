use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetStickerPositionInSet {
    sticker: String,
    position: Integer,
}