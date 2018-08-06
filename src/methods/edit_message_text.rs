use super::super::types::*;

/// Use this method to edit text and game messages sent by the bot or via the bot (for inline bots). On success, if edited message is sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct EditMessageText {
    chat_id: Option<ChatId>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
    text: String,
    parse_mode: Option<String>,
    disable_web_page_preview: Option<bool>,
    reply_markup: Option<InlineKeyboardMarkup>,
}