use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultContact {
    ty: String,
    id: String,
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    vcard: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
    input_message_content: Option<InputMessageContent>,
    thumb_url: Option<String>,
    thumb_width: Option<Integer>,
    thumb_height: Option<Integer>,
}