use super::*;

/// This object contains information about one member of a chat.
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMember {
    user: User,
    status: String,
    until_date: Option<Integer>,
    can_be_edited: Option<bool>,
    can_change_info: Option<bool>,
    can_post_messages: Option<bool>,
    can_edit_messages: Option<bool>,
    can_delete_messages: Option<bool>,
    can_invite_users: Option<bool>,
    can_restrict_members: Option<bool>,
    can_pin_messages: Option<bool>,
    can_promote_members: Option<bool>,
    can_send_messages: Option<bool>,
    can_send_media_messages: Option<bool>,
    can_send_other_messages: Option<bool>,
    can_add_web_page_previews: Option<bool>,
}