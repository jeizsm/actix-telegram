use types::*;

/// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of InlineKeyboardButton objects
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
