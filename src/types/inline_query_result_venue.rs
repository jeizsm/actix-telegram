use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultVenue {
    ty: String,
    id: String,
    latitude: Float,
    longitude: Float,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumb_url: Option<String>,
    thumb_width: Option<Integer>,
    thumb_height: Option<Integer>,
}