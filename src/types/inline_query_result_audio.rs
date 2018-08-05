use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultAudio {
    ty: String,
    id: String,
    audio_url: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    performer: Option<String>,
    audio_duration: Option<Integer>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}