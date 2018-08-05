use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}