use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGameHighScores {
    user_id: Integer,
    chat_id: Option<Integer>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
}