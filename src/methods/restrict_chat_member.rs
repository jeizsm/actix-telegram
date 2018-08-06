use super::super::types::*;

/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate admin rights. Pass True for all boolean parameters to lift restrictions from a user. Returns True on success.
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