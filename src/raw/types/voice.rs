use crate::types::*;

/// This object represents a voice note.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct Voice {
    /// Unique identifier for this file
    pub(crate) file_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub(crate) duration: Integer,
    /// MIME type of the file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mime_type: Option<String>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file_size: Option<Integer>,
}
