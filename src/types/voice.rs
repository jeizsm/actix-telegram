use super::*;

/// This object represents a voice note.
#[derive(Serialize, Deserialize, Debug)]
pub struct Voice {
    file_id: String,
    duration: Integer,
    mime_type: Option<String>,
    file_size: Option<Integer>,
}