use crate::types::*;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum InputMedia {
    InputMediaAnimation(InputMediaAnimation),
    InputMediaDocument(InputMediaDocument),
    InputMediaAudio(InputMediaAudio),
    InputMediaPhoto(InputMediaPhoto),
    InputMediaVideo(InputMediaVideo),
}