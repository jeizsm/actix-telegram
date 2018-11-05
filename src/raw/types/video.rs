use types::*;

/// This object represents a video file.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct Video {
    /// Unique identifier for this file
    file_id: String,
    /// Video width as defined by sender
    width: Integer,
    /// Video height as defined by sender
    height: Integer,
    /// Duration of the video in seconds as defined by sender
    duration: Integer,
    /// Video thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<PhotoSize>,
    /// Mime type of a file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    mime_type: Option<String>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    file_size: Option<Integer>,
}
