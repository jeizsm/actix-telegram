use super::*;

/// This object represents a sticker.
#[derive(Serialize, Deserialize, Debug)]
pub struct Sticker {
    file_id: String,
    width: Integer,
    height: Integer,
    thumb: Option<PhotoSize>,
    emoji: Option<String>,
    set_name: Option<String>,
    mask_position: Option<MaskPosition>,
    file_size: Option<Integer>,
}