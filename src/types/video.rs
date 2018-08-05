use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    file_id: String,
    width: Integer,
    height: Integer,
    duration: Integer,
    thumb: Option<PhotoSize>,
    mime_type: Option<String>,
    file_size: Option<Integer>,
}