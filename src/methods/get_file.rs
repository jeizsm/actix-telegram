use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFile {
    file_id: String,
}