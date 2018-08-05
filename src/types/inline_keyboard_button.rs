use super::*;

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