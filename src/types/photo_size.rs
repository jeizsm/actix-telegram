use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoSize {
    file_id: String,
    width: Integer,
    height: Integer,
    file_size: Option<Integer>,
}