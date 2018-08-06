use super::*;

/// Represents a Game.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultGame {
    ty: String,
    id: String,
    game_short_name: String,
    reply_markup: Option<InlineKeyboardMarkup>,
}