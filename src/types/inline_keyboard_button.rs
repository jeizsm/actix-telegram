use super::*;

/// This object represents one button of an inline keyboard. You must use exactly one of the optional fields.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardButton {
    text: String,
    url: Option<String>,
    callback_data: Option<String>,
    switch_inline_query: Option<String>,
    switch_inline_query_current_chat: Option<String>,
    callback_game: Option<CallbackGame>,
    pay: Option<bool>,
}