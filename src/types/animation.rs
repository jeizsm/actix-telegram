use super::*;

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Serialize, Deserialize, Debug)]
pub struct Animation {
    file_id: String,
    width: Integer,
    height: Integer,
    duration: Integer,
    thumb: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<Integer>,
}