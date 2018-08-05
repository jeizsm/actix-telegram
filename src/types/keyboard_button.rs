use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardButton {
    text: String,
    request_contact: Option<bool>,
    request_location: Option<bool>,
}