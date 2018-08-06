use super::super::types::*;

/// Use this method to edit live location messages sent by the bot or via the bot (for inline bots). A location can be edited until its live_period expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if the edited message was sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct EditMessageLiveLocation {
    chat_id: Option<ChatId>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
    latitude: Float,
    longitude: Float,
    reply_markup: Option<InlineKeyboardMarkup>,
}