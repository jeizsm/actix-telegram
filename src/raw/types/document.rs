use crate::types::*;

/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct Document {
    /// Unique file identifier
    pub(crate) file_id: String,
    /// Document thumbnail as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thumb: Option<PhotoSize>,
    /// Original filename as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file_name: Option<String>,
    /// MIME type of the file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mime_type: Option<String>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file_size: Option<Integer>,
}