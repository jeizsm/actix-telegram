use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultCachedAudio {
    ty: String,
    id: String,
    audio_file_id: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}