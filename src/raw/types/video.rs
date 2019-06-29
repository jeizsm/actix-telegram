use crate::types::*;

/// This object represents a video file.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct Video {
    /// Unique identifier for this file
    pub(crate) file_id: String,
    /// Video width as defined by sender
    pub(crate) width: Integer,
    /// Video height as defined by sender
    pub(crate) height: Integer,
    /// Duration of the video in seconds as defined by sender
    pub(crate) duration: Integer,
    /// Video thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb: Option<PhotoSize>,
    /// Mime type of a file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mime_type: Option<String>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file_size: Option<Integer>,
}