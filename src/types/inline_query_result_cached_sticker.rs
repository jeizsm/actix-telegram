use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedSticker {
    ty: String,
    id: String,
    sticker_file_id: String,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}