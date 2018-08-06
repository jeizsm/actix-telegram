use super::*;

/// This object represents one button of the reply keyboard. For simple text buttons String can be used instead of this object to specify text of the button. Optional fields are mutually exclusive.
#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardButton {
    text: String,
    request_contact: Option<bool>,
    request_location: Option<bool>,
}