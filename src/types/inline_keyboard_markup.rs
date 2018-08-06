use super::*;

/// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}