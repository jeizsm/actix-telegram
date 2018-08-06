use super::*;

/// Represents a link to an mp3 audio file stored on the Telegram servers. By default, this audio file will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
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