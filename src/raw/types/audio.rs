use crate::types::*;

/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct Audio {
    /// Unique identifier for this file
    file_id: String,
    /// Duration of the audio in seconds as defined by sender
    duration: Integer,
    /// Performer of the audio as defined by sender or by audio tags
    #[serde(skip_serializing_if = "Option::is_none")]
    performer: Option<String>,
    /// Title of the audio as defined by sender or by audio tags
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    /// MIME type of the file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    mime_type: Option<String>,
    /// File size
    #[serde(skip_serializing_if = "Option::is_none")]
    file_size: Option<Integer>,
    /// Thumbnail of the album cover to which the music file belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb: Option<PhotoSize>,
}
