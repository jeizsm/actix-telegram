use super::*;

/// This object represents an incoming update.At most one of the optional parameters can be present in any given update.
#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    update_id: Integer,
    message: Option<Message>,
    edited_message: Option<Message>,
    channel_post: Option<Message>,
    edited_channel_post: Option<Message>,
    inline_query: Option<InlineQuery>,
    chosen_inline_result: Option<ChosenInlineResult>,
    callback_query: Option<CallbackQuery>,
    shipping_query: Option<ShippingQuery>,
    pre_checkout_query: Option<PreCheckoutQuery>,
}