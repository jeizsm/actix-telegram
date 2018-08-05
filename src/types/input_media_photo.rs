use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InputMediaPhoto {
    ty: String,
    media: String,
    caption: Option<String>,
    parse_mode: Option<String>,
}