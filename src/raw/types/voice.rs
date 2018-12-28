use crate::types::*;

/// This object represents a voice note.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct Voice {
    /// Unique identifier for this file
    file_id: String,
    /// Duration of the audio in seconds as defined by sender
    duration: Integer,
    /// MIME type of the file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    mime_type: Option<String>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    file_size: Option<Integer>,
}
