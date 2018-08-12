use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum InputMedia {
    InputMediaAnimation(InputMediaAnimation),
    InputMediaDocument(InputMediaDocument),
    InputMediaAudio(InputMediaAudio),
    InputMediaPhoto(InputMediaPhoto),
    InputMediaVideo(InputMediaVideo),
}
