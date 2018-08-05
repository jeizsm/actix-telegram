use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PassportElementErrorFiles {
    source: String,
    ty: String,
    file_hashes: Vec<String>,
    message: String,
}