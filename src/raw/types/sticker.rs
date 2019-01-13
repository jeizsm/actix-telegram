use crate::types::*;

/// This object represents a sticker.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct Sticker {
    /// Unique identifier for this file
    pub(crate) file_id: String,
    /// Sticker width
    pub(crate) width: Integer,
    /// Sticker height
    pub(crate) height: Integer,
    /// Sticker thumbnail in the .webp or .jpg format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb: Option<PhotoSize>,
    /// Emoji associated with the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) emoji: Option<String>,
    /// Name of the sticker set to which the sticker belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) set_name: Option<String>,
    /// For mask stickers, the position where the mask should be placed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mask_position: Option<MaskPosition>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file_size: Option<Integer>,
}
