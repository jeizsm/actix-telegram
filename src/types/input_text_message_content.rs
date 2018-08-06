use super::*;

/// Represents the content of a text message to be sent as the result of an inline query. 
#[derive(Serialize, Deserialize, Debug)]
pub struct InputTextMessageContent {
    message_text: String,
    parse_mode: Option<String>,
    disable_web_page_preview: Option<bool>,
}