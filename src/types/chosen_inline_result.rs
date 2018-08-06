use super::*;

/// Represents a result of an inline query that was chosen by the user and sent to their chat partner. 
#[derive(Serialize, Deserialize, Debug)]
pub struct ChosenInlineResult {
    result_id: String,
    from: User,
    location: Option<Location>,
    inline_message_id: Option<String>,
    query: String,
}