use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    user_id: Option<Integer>,
    vcard: Option<String>,
}