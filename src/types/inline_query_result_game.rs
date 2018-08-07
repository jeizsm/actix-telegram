use super::*;

/// Represents a Game.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultGame {
    /// Type of the result, must be game
    #[serde(rename = "type")]
    pub type_: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
