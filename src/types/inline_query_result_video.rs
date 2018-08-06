use super::*;

/// 
/// If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube), you must replace its content using input_message_content.
/// 
/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultVideo {
    ty: String,
    id: String,
    video_url: String,
    mime_type: String,
    thumb_url: String,
    title: String,
    caption: Option<String>,
    parse_mode: Option<String>,
    video_width: Option<Integer>,
    video_height: Option<Integer>,
    video_duration: Option<Integer>,
    description: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}