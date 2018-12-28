use crate::types::True;

#[derive(Serialize, Debug, TelegramApi)]
#[return_type = "True"]
pub struct DeleteWebhook;
