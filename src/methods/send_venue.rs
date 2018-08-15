use super::*;

/// Use this method to send information about a venue. On success, the sent Message is returned.
#[derive(Serialize, Debug, TelegramApi)]
#[return_type = "Message"]
pub struct SendVenue {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: ChatId,
    /// Latitude of the venue
    pub latitude: Float,
    /// Longitude of the venue
    pub longitude: Float,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<Integer>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}
