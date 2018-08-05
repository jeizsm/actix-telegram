use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackQuery {
    id: String,
    from: User,
    message: Option<Message>,
    inline_message_id: Option<String>,
    chat_instance: String,
    data: Option<String>,
    game_short_name: Option<String>,
}