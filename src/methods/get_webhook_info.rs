use crate::types::WebhookInfo;

#[derive(Serialize, Debug, TelegramApi)]
#[return_type = "WebhookInfo"]
pub struct GetWebhookInfo;
