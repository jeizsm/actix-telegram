use crate::types::*;

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct Animation {
    /// Unique file identifier
    pub(crate) file_id: String,
    /// Video width as defined by sender
    pub(crate) width: Integer,
    /// Video height as defined by sender
    pub(crate) height: Integer,
    /// Duration of the video in seconds as defined by sender
    pub(crate) duration: Integer,
    /// Animation thumbnail as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb: Option<PhotoSize>,
    /// Original animation filename as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file_name: Option<String>,
    /// MIME type of the file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mime_type: Option<String>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file_size: Option<Integer>,
}