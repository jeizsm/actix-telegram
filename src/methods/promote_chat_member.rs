use super::super::types::*;

/// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Pass False for all boolean parameters to demote a user. Returns True on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct PromoteChatMember {
    chat_id: ChatId,
    user_id: Integer,
    can_change_info: Option<bool>,
    can_post_messages: Option<bool>,
    can_edit_messages: Option<bool>,
    can_delete_messages: Option<bool>,
    can_invite_users: Option<bool>,
    can_restrict_members: Option<bool>,
    can_pin_messages: Option<bool>,
    can_promote_members: Option<bool>,
}