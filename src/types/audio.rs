use super::*;

/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {
    file_id: String,
    duration: Integer,
    performer: Option<String>,
    title: Option<String>,
    mime_type: Option<String>,
    file_size: Option<Integer>,
    thumb: Option<PhotoSize>,
}