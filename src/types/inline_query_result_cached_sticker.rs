use super::*;

/// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the sticker.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedSticker {
    ty: String,
    id: String,
    sticker_file_id: String,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}