use super::*;

/// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
#[derive(Serialize, Deserialize, Debug)]
pub struct PassportElementErrorDataField {
    source: String,
    ty: String,
    field_name: String,
    data_hash: String,
    message: String,
}