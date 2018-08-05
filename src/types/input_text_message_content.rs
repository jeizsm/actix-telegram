use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InputTextMessageContent {
    message_text: String,
    parse_mode: Option<String>,
    disable_web_page_preview: Option<bool>,
}