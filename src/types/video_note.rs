use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoNote {
    file_id: String,
    length: Integer,
    duration: Integer,
    thumb: Option<PhotoSize>,
    file_size: Option<Integer>,
}