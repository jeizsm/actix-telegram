use types::*;

/// This object contains information about one member of a chat.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct ChatMember {
    /// Information about the user
    user: User,
    /// The member's status in the chat. Can be “creator”, “administrator”, “member”, “restricted”, “left” or “kicked”
    status: String,
    /// Restricted and kicked only. Date when restrictions will be lifted for this user, unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<Integer>,
    /// Administrators only. True, if the bot is allowed to edit administrator privileges of that user
    #[serde(skip_serializing_if = "Option::is_none")]
    can_be_edited: Option<bool>,
    /// Administrators only. True, if the administrator can change the chat title, photo and other settings
    #[serde(skip_serializing_if = "Option::is_none")]
    can_change_info: Option<bool>,
    /// Administrators only. True, if the administrator can post in the channel, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    can_post_messages: Option<bool>,
    /// Administrators only. True, if the administrator can edit messages of other users and can pin messages, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    can_edit_messages: Option<bool>,
    /// Administrators only. True, if the administrator can delete messages of other users
    #[serde(skip_serializing_if = "Option::is_none")]
    can_delete_messages: Option<bool>,
    /// Administrators only. True, if the administrator can invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    can_invite_users: Option<bool>,
    /// Administrators only. True, if the administrator can restrict, ban or unban chat members
    #[serde(skip_serializing_if = "Option::is_none")]
    can_restrict_members: Option<bool>,
    /// Administrators only. True, if the administrator can pin messages, supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    can_pin_messages: Option<bool>,
    /// Administrators only. True, if the administrator can add new administrators with a subset of his own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    #[serde(skip_serializing_if = "Option::is_none")]
    can_promote_members: Option<bool>,
    /// Restricted only. True, if the user can send text messages, contacts, locations and venues
    #[serde(skip_serializing_if = "Option::is_none")]
    can_send_messages: Option<bool>,
    /// Restricted only. True, if the user can send audios, documents, photos, videos, video notes and voice notes, implies can_send_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    can_send_media_messages: Option<bool>,
    /// Restricted only. True, if the user can send animations, games, stickers and use inline bots, implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    can_send_other_messages: Option<bool>,
    /// Restricted only. True, if user may add web page previews to his messages, implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    can_add_web_page_previews: Option<bool>,
}
