use crate::types::*;

/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct VideoNote {
    /// Unique identifier for this file
    pub(crate) file_id: String,
    /// Video width and height (diameter of the video message) as defined by sender
    pub(crate) length: Integer,
    /// Duration of the video in seconds as defined by sender
    pub(crate) duration: Integer,
    /// Video thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb: Option<PhotoSize>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file_size: Option<Integer>,
}