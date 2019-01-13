use crate::types::*;

/// This object represents a phone contact.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct Contact {
    /// Contact's phone number
    pub(crate) phone_number: String,
    /// Contact's first name
    pub(crate) first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) last_name: Option<String>,
    /// Contact's user identifier in Telegram
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) user_id: Option<Integer>,
    /// Additional data about the contact in the form of a vCard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) vcard: Option<String>,
}
