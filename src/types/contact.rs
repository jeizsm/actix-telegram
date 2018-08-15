use types::*;

/// This object represents a phone contact.
#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Contact's user identifier in Telegram
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Integer>,
    /// Additional data about the contact in the form of a vCard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}
