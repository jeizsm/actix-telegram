use super::*;

/// Represents a link to an mp3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
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