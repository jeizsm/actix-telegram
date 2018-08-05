use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyKeyboardRemove {
    remove_keyboard: True,
    selective: Option<bool>,
}