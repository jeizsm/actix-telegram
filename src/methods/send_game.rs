use super::*;

/// Use this method to send a game. On success, the sent Message is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendGame {
    /// Unique identifier for the target chat
    pub chat_id: Integer,
    /// Short name of the game, serves as the unique identifier for the game. Set up your games via Botfather.
    pub game_short_name: String,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<Integer>,
    /// A JSON-serialized object for an inline keyboard. If empty, one ‘Play game_title’ button will be shown. If not empty, the first button must launch the game.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
