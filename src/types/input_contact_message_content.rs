/// Represents the content of a contact message to be sent as the result of an inline query.
#[derive(Serialize, Deserialize, Debug)]
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub vcard: Option<String>,
}
