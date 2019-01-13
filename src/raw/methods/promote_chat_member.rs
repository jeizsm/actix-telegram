use crate::types::*;

/// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Pass False for all boolean parameters to demote a user. Returns True on success.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "True"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct PromoteChatMember {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub(crate) chat_id: ChatIdOrUsername,
    /// Unique identifier of the target user
    pub(crate) user_id: Integer,
    /// Pass True, if the administrator can change chat title, photo and other settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) can_change_info: Option<bool>,
    /// Pass True, if the administrator can create channel posts, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) can_post_messages: Option<bool>,
    /// Pass True, if the administrator can edit messages of other users and can pin messages, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) can_edit_messages: Option<bool>,
    /// Pass True, if the administrator can delete messages of other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) can_delete_messages: Option<bool>,
    /// Pass True, if the administrator can invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) can_invite_users: Option<bool>,
    /// Pass True, if the administrator can restrict, ban or unban chat members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) can_restrict_members: Option<bool>,
    /// Pass True, if the administrator can pin messages, supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) can_pin_messages: Option<bool>,
    /// Pass True, if the administrator can add new administrators with a subset of his own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by him)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) can_promote_members: Option<bool>,
}
