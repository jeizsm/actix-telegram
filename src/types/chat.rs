use super::*;

/// This object represents a chat.
#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    id: Integer,
    ty: String,
    title: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    all_members_are_administrators: Option<bool>,
    photo: Option<ChatPhoto>,
    description: Option<String>,
    invite_link: Option<String>,
    pinned_message: Box<Option<Message>>,
    sticker_set_name: Option<String>,
    can_set_sticker_set: Option<bool>,
}