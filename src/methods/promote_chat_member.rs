use super::super::types::*;

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