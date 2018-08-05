use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetGameScore {
    user_id: Integer,
    score: Integer,
    force: Option<bool>,
    disable_edit_message: Option<bool>,
    chat_id: Option<Integer>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
}