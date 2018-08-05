use super::*;

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