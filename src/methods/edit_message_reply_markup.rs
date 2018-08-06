use super::super::types::*;

/// Use this method to edit only the reply markup of messages sent by the bot or via the bot (for inline bots).  On success, if edited message is sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct EditMessageReplyMarkup {
    chat_id: Option<ChatId>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
}