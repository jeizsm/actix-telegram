use types::*;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum InputMediaPhotoOrInputMediaVideo {
    InputMediaPhoto(InputMediaPhoto),
    InputMediaVideo(InputMediaVideo),
}
