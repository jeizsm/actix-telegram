use crate::types::*;

/// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Debug, Deserialize, Clone, Getters, Serialize, Setters, New)]
#[get(vis = "pub")]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of InlineKeyboardButton objects
    pub(crate) inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}