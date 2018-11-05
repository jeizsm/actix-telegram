use types::*;

/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct PhotoSize {
    /// Unique identifier for this file
    file_id: String,
    /// Photo width
    width: Integer,
    /// Photo height
    height: Integer,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    file_size: Option<Integer>,
}
