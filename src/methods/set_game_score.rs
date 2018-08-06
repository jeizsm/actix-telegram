use super::super::types::*;

/// Use this method to set the score of the specified user in a game. On success, if the message was sent by the bot, returns the edited Message, otherwise returns True. Returns an error, if the new score is not greater than the user's current score in the chat and force is False.
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