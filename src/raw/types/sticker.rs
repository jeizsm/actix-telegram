use crate::types::*;

/// This object represents a sticker.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct Sticker {
    /// Unique identifier for this file
    file_id: String,
    /// Sticker width
    width: Integer,
    /// Sticker height
    height: Integer,
    /// Sticker thumbnail in the .webp or .jpg format
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<PhotoSize>,
    /// Emoji associated with the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<String>,
    /// Name of the sticker set to which the sticker belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    set_name: Option<String>,
    /// For mask stickers, the position where the mask should be placed
    #[serde(skip_serializing_if = "Option::is_none")]
    mask_position: Option<MaskPosition>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    file_size: Option<Integer>,
}
