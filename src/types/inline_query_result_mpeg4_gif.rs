use super::*;

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultMpeg4Gif {
    ty: String,
    id: String,
    mpeg4_url: String,
    mpeg4_width: Option<Integer>,
    mpeg4_height: Option<Integer>,
    mpeg4_duration: Option<Integer>,
    thumb_url: String,
    title: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}