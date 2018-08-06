use super::*;

/// Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultPhoto {
    ty: String,
    id: String,
    photo_url: String,
    thumb_url: String,
    photo_width: Option<Integer>,
    photo_height: Option<Integer>,
    title: Option<String>,
    description: Option<String>,
    caption: Option<String>,
    parse_mode: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
}