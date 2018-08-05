use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PassportElementErrorSelfie {
    source: String,
    ty: String,
    file_hash: String,
    message: String,
}