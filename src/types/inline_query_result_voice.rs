use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultVoice {
    ty: String,
    id: String,
    voice_url: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    voice_duration: Option<Integer>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}