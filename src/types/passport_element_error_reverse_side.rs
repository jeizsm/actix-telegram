use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PassportElementErrorReverseSide {
    source: String,
    ty: String,
    file_hash: String,
    message: String,
}