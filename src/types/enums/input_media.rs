use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum InputMedia {
    InputMediaAnimation(InputMediaAnimation),
    InputMediaDocument(InputMediaDocument),
    InputMediaAudio(InputMediaAudio),
    InputMediaPhoto(InputMediaPhoto),
    InputMediaVideo(InputMediaVideo),
}