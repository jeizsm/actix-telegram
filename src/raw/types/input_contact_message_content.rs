use crate::types::*;

/// Represents the content of a contact message to be sent as the result of an inline query.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub(crate) phone_number: String,
    /// Contact's first name
    pub(crate) first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) vcard: Option<String>,
}
