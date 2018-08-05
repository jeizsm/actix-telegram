use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PassportElementErrorFile {
    source: String,
    ty: String,
    file_hash: String,
    message: String,
}