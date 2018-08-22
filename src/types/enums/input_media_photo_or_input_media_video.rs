use types::*;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum InputMediaPhotoOrInputMediaVideo {
    InputMediaPhoto(InputMediaPhoto),
    InputMediaVideo(InputMediaVideo),
}
