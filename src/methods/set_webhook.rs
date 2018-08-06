use super::super::types::*;

/// If you'd like to make sure that the Webhook request comes from Telegram, we recommend using a secret path in the URL, e.g. https://www.example.com/<token>. Since nobody else knows your bot‘s token, you can be pretty sure it’s us.
/// Use this method to specify a url and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified url, containing a JSON-serialized Update. In case of an unsuccessful request, we will give up after a reasonable amount of attempts. Returns True on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct SetWebhook {
    url: String,
    certificate: Option<InputFile>,
    max_connections: Option<Integer>,
    allowed_updates: Option<Vec<String>>,
}