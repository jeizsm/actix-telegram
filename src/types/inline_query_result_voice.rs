use super::*;

/// Represents a link to a voice recording in an .ogg container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the the voice message.
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