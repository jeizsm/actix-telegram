use super::super::types::*;

/// Use this method to send invoices. On success, the sent Message is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendInvoice {
    chat_id: Integer,
    title: String,
    description: String,
    payload: String,
    provider_token: String,
    start_parameter: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    provider_data: Option<String>,
    photo_url: Option<String>,
    photo_size: Option<Integer>,
    photo_width: Option<Integer>,
    photo_height: Option<Integer>,
    need_name: Option<bool>,
    need_phone_number: Option<bool>,
    need_email: Option<bool>,
    need_shipping_address: Option<bool>,
    send_phone_number_to_provider: Option<bool>,
    send_email_to_provider: Option<bool>,
    is_flexible: Option<bool>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<InlineKeyboardMarkup>,
}