use types::*;

/// Use this method to edit text and game messages sent by the bot or via the bot (for inline bots). On success, if edited message is sent by the bot, the edited Message is returned, otherwise True is returned.
#[derive(Debug, Serialize, TelegramApi)]
#[return_type = "MessageOrTrue"]
pub struct EditMessageText {
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatIdOrUsername>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Integer>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// New text of the message
    pub text: String,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Disables link previews for links in this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    /// A JSON-serialized object for an inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
