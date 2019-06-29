use crate::types::*;

/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct PhotoSize {
    /// Unique identifier for this file
    pub(crate) file_id: String,
    /// Photo width
    pub(crate) width: Integer,
    /// Photo height
    pub(crate) height: Integer,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file_size: Option<Integer>,
}