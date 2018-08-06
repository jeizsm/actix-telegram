use super::*;

/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Serialize, Deserialize, Debug)]
pub struct PhotoSize {
    file_id: String,
    width: Integer,
    height: Integer,
    file_size: Option<Integer>,
}