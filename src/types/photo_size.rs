use types::*;

/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoSize {
    /// Unique identifier for this file
    pub file_id: String,
    /// Photo width
    pub width: Integer,
    /// Photo height
    pub height: Integer,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<Integer>,
}
