use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PassportElementErrorFrontSide {
    source: String,
    ty: String,
    file_hash: String,
    message: String,
}