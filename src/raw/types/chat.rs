use crate::types::*;

/// This object represents a chat.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct Chat {
    /// Unique identifier for this chat. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    id: Integer,
    /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    type_: String,
    /// Title, for supergroups, channels and group chats
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    /// First name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<String>,
    /// Last name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    /// True if a group has ‘All Members Are Admins’ enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    all_members_are_administrators: Option<bool>,
    /// Chat photo. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    photo: Option<ChatPhoto>,
    /// Description, for supergroups and channel chats. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// Chat invite link, for supergroups and channel chats. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    invite_link: Option<String>,
    /// Pinned message, for supergroups and channel chats. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pinned_message: Option<Box<Message>>,
    /// For supergroups, name of group sticker set. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    can_set_sticker_set: Option<bool>,
}
