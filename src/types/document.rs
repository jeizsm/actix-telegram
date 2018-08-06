use super::*;

/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    file_id: String,
    thumb: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<Integer>,
}