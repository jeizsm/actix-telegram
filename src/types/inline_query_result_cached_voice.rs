use super::*;

/// Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the voice message.
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