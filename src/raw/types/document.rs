use types::*;

/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct Document {
    /// Unique file identifier
    file_id: String,
    /// Document thumbnail as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<PhotoSize>,
    /// Original filename as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    file_name: Option<String>,
    /// MIME type of the file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    mime_type: Option<String>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    file_size: Option<Integer>,
}
