use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InputContactMessageContent {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    vcard: Option<String>,
}