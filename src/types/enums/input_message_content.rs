use super::super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum InputMessageContent {
    InputTextMessageContent(InputTextMessageContent),
    InputLocationMessageContent(InputLocationMessageContent),
    InputVenueMessageContent(InputVenueMessageContent),
    InputContactMessageContent(InputContactMessageContent),
}