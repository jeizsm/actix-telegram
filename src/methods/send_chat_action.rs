use super::super::types::*;

/// We only recommend using this method when a response from the bot will take a noticeable amount of time to arrive.
/// 
/// Example: The ImageBot needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use sendChatAction with action = upload_photo. The user will see a “sending photo” status for the bot.
/// 
/// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns True on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendChatAction {
    chat_id: ChatId,
    action: String,
}