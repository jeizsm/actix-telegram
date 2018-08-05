use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedVoice {
    ty: String,
    id: String,
    voice_file_id: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}