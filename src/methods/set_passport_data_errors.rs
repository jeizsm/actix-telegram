use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetPassportDataErrors {
    user_id: Integer,
    errors: Vec<PassportElementError>,
}