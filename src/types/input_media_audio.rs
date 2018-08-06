use super::*;

/// Represents an audio file to be treated as music to be sent.
#[derive(Serialize, Deserialize, Debug)]
pub struct InputMediaAudio {
    ty: String,
    media: String,
    thumb: Option<Thumb>,
    caption: Option<String>,
    parse_mode: Option<String>,
    duration: Option<Integer>,
    performer: Option<String>,
    title: Option<String>,
}