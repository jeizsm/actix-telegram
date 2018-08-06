use super::*;

/// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the location.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultLocation {
    ty: String,
    id: String,
    latitude: Float,
    longitude: Float,
    title: String,
    live_period: Option<Integer>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumb_url: Option<String>,
    thumb_width: Option<Integer>,
    thumb_height: Option<Integer>,
}