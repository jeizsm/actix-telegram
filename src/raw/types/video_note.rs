use crate::types::*;

/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct VideoNote {
    /// Unique identifier for this file
    file_id: String,
    /// Video width and height (diameter of the video message) as defined by sender
    length: Integer,
    /// Duration of the video in seconds as defined by sender
    duration: Integer,
    /// Video thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<PhotoSize>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    file_size: Option<Integer>,
}
