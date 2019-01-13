use crate::types::*;

/// Contains information about why a request was unsuccessful.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct ResponseParameters {
    /// The group has been migrated to a supergroup with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) migrate_to_chat_id: Option<Integer>,
    /// In case of exceeding flood control, the number of seconds left to wait before the request can be repeated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) retry_after: Option<Integer>,
}
