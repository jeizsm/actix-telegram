use super::super::types::*;

/// Use this method to stop updating a live location message sent by the bot or via the bot (for inline bots) before live_period expires. On success, if the message was sent by the bot, the sent Message is returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct StopMessageLiveLocation {
    chat_id: Option<ChatId>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
}