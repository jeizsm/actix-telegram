use super::*;

/// This object represents a chat photo.
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatPhoto {
    small_file_id: String,
    big_file_id: String,
}