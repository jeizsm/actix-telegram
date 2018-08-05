use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RestrictChatMember {
    chat_id: ChatId,
    user_id: Integer,
    until_date: Option<Integer>,
    can_send_messages: Option<bool>,
    can_send_media_messages: Option<bool>,
    can_send_other_messages: Option<bool>,
    can_add_web_page_previews: Option<bool>,
}