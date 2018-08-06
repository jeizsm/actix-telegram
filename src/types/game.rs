use super::*;

/// This object represents a game. Use BotFather to create and edit games, their short names will act as unique identifiers.
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    title: String,
    description: String,
    photo: Vec<PhotoSize>,
    text: Option<String>,
    text_entities: Option<Vec<MessageEntity>>,
    animation: Option<Animation>,
}