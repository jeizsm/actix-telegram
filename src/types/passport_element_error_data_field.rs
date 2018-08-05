use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PassportElementErrorDataField {
    source: String,
    ty: String,
    field_name: String,
    data_hash: String,
    message: String,
}