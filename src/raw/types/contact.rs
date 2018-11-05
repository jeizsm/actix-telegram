use types::*;

/// This object represents a phone contact.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct Contact {
    /// Contact's phone number
    phone_number: String,
    /// Contact's first name
    first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    /// Contact's user identifier in Telegram
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<Integer>,
    /// Additional data about the contact in the form of a vCard
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<String>,
}
