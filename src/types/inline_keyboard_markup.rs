use super::*;

/// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of InlineKeyboardButton objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
