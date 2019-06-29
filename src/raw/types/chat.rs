use crate::types::*;

/// This object represents a chat.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct Chat {
    /// Unique identifier for this chat. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    pub(crate) id: Integer,
    /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// Title, for supergroups, channels and group chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) username: Option<String>,
    /// First name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) first_name: Option<String>,
    /// Last name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) last_name: Option<String>,
    /// True if a group has ‘All Members Are Admins’ enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) all_members_are_administrators: Option<bool>,
    /// Chat photo. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) photo: Option<ChatPhoto>,
    /// Description, for supergroups and channel chats. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    /// Chat invite link, for supergroups and channel chats. Each administrator in a chat generates their own invite links, so the bot must first generate the link using exportChatInviteLink. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) invite_link: Option<String>,
    /// Pinned message, for groups, supergroups and channels. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pinned_message: Option<Box<Message>>,
    /// For supergroups, name of group sticker set. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) can_set_sticker_set: Option<bool>,
}