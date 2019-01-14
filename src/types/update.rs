use crate::types::{
    CallbackQuery, ChosenInlineResult, InlineQuery, Message, PreCheckoutQuery, ShippingQuery,
    UpdateId,
};

/// This object represents an incoming update.At most one of the optional parameters can be present in any given update.
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Update {
    Message(MessageUpdate),
    EditedMessage(EditedMessageUpdate),
    ChannelPost(ChannelPostUpdate),
    EditedChannelPost(EditedChannelPostUpdate),
    InlineQuery(InlineQueryUpdate),
    ChosenInlineResult(ChosenInlineResultUpdate),
    CallbackQuery(CallbackQueryUpdate),
    ShippingQuery(ShippingQueryUpdate),
    PreCheckoutQuery(PreCheckoutQueryUpdate),
    Unknown(UnknownUpdate),
}

#[derive(Debug, Clone)]
pub enum UpdateKind<'a> {
    /// New incoming message of any kind — text, photo, sticker, etc.
    Message(&'a Message),
    /// New version of a message that is known to the bot and was edited
    EditedMessage(&'a Message),
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    ChannelPost(&'a Message),
    /// New version of a channel post that is known to the bot and was edited
    EditedChannelPost(&'a Message),
    /// New incoming inline query
    InlineQuery(&'a InlineQuery),
    /// The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    ChosenInlineResult(&'a ChosenInlineResult),
    /// New incoming callback query
    CallbackQuery(&'a CallbackQuery),
    /// New incoming shipping query. Only for invoices with flexible price
    ShippingQuery(&'a ShippingQuery),
    /// New incoming pre-checkout query. Contains full information about checkout
    PreCheckoutQuery(&'a PreCheckoutQuery),
    /// Uknown update. Should not happen
    Unknown,
}

impl Update {
    #[inline]
    /// The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you’re using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub fn update_id(&self) -> UpdateId {
        match self {
            Update::Message(update) => update.update_id,
            Update::EditedMessage(update) => update.update_id,
            Update::ChannelPost(update) => update.update_id,
            Update::EditedChannelPost(update) => update.update_id,
            Update::InlineQuery(update) => update.update_id,
            Update::ChosenInlineResult(update) => update.update_id,
            Update::CallbackQuery(update) => update.update_id,
            Update::ShippingQuery(update) => update.update_id,
            Update::PreCheckoutQuery(update) => update.update_id,
            Update::Unknown(update) => update.update_id,
        }
    }

    #[inline]
    pub fn kind(&self) -> UpdateKind {
        match self {
            Update::Message(update) => UpdateKind::Message(&update.message),
            Update::EditedMessage(update) => UpdateKind::EditedMessage(&update.edited_message),
            Update::ChannelPost(update) => UpdateKind::ChannelPost(&update.channel_post),
            Update::EditedChannelPost(update) => {
                UpdateKind::EditedChannelPost(&update.edited_channel_post)
            }
            Update::InlineQuery(update) => UpdateKind::InlineQuery(&update.inline_query),
            Update::ChosenInlineResult(update) => {
                UpdateKind::ChosenInlineResult(&update.chosen_inline_result)
            }
            Update::CallbackQuery(update) => UpdateKind::CallbackQuery(&update.callback_query),
            Update::ShippingQuery(update) => UpdateKind::ShippingQuery(&update.shipping_query),
            Update::PreCheckoutQuery(update) => {
                UpdateKind::PreCheckoutQuery(&update.pre_checkout_query)
            }
            Update::Unknown(_) => UpdateKind::Unknown,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct MessageUpdate {
    /// The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you’re using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: UpdateId,
    /// New incoming message of any kind — text, photo, sticker, etc.
    pub message: Message,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EditedMessageUpdate {
    /// The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you’re using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: UpdateId,
    /// New version of a message that is known to the bot and was edited
    pub edited_message: Message,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChannelPostUpdate {
    /// The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you’re using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: UpdateId,
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    pub channel_post: Message,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EditedChannelPostUpdate {
    /// The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you’re using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: UpdateId,
    /// New version of a channel post that is known to the bot and was edited
    pub edited_channel_post: Message,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InlineQueryUpdate {
    /// The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you’re using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: UpdateId,
    /// New incoming inline query
    pub inline_query: InlineQuery,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChosenInlineResultUpdate {
    /// The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you’re using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: UpdateId,
    /// The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    pub chosen_inline_result: ChosenInlineResult,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CallbackQueryUpdate {
    /// The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you’re using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: UpdateId,
    /// New incoming callback query
    pub callback_query: CallbackQuery,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ShippingQueryUpdate {
    /// The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you’re using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: UpdateId,
    /// New incoming shipping query. Only for invoices with flexible price
    pub shipping_query: ShippingQuery,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PreCheckoutQueryUpdate {
    /// The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you’re using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: UpdateId,
    /// New incoming pre-checkout query. Contains full information about checkout
    pub pre_checkout_query: PreCheckoutQuery,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UnknownUpdate {
    /// The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you’re using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: UpdateId,
}
