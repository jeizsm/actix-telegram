use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Voice {
    file_id: String,
    duration: Integer,
    mime_type: Option<String>,
    file_size: Option<Integer>,
}