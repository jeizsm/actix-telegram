use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    file_id: String,
    file_size: Option<Integer>,
    file_path: Option<String>,
}