use super::super::types::*;

/// Use this method to get up to date information about the chat (current name of the user for one-on-one conversations, current username of a user, group or channel, etc.). Returns a Chat object on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetChat {
    chat_id: ChatId,
}