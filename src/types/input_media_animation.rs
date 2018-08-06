use super::*;

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(Serialize, Deserialize, Debug)]
pub struct InputMediaAnimation {
    ty: String,
    media: String,
    thumb: Option<Thumb>,
    caption: Option<String>,
    parse_mode: Option<String>,
    width: Option<Integer>,
    height: Option<Integer>,
    duration: Option<Integer>,
}