use super::*;

/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Serialize, Deserialize, Debug)]
pub struct VideoNote {
    file_id: String,
    length: Integer,
    duration: Integer,
    thumb: Option<PhotoSize>,
    file_size: Option<Integer>,
}