use crate::types::User;

#[derive(Serialize, Debug, TelegramApi)]
#[return_type = "User"]
pub struct GetMe;
