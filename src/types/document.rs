use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    file_id: String,
    thumb: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<Integer>,
}