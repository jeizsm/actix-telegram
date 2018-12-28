use crate::types::*;

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct Animation {
    /// Unique file identifier
    file_id: String,
    /// Video width as defined by sender
    width: Integer,
    /// Video height as defined by sender
    height: Integer,
    /// Duration of the video in seconds as defined by sender
    duration: Integer,
    /// Animation thumbnail as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<PhotoSize>,
    /// Original animation filename as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    file_name: Option<String>,
    /// MIME type of the file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    mime_type: Option<String>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    file_size: Option<Integer>,
}
