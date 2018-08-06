use super::super::types::*;

/// Use this method to edit audio, document, photo, or video messages. If a message is a part of a message album, then it can be edited only to a photo or a video. Otherwise, message type can be changed arbitrarily. When inline message is edited, new file can't be uploaded. Use previously uploaded file via its file_id or specify a URL. On success, if the edited message was sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct EditMessageMedia {
    chat_id: Option<ChatId>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
    media: InputMedia,
    reply_markup: Option<InlineKeyboardMarkup>,
}