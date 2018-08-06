use super::*;

/// This object represents a custom keyboard with reply options (see Introduction to bots for details and examples).
#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyKeyboardMarkup {
    keyboard: Vec<Vec<KeyboardButton>>,
    resize_keyboard: Option<bool>,
    one_time_keyboard: Option<bool>,
    selective: Option<bool>,
}