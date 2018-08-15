use types::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum InputMediaPhotoOrInputMediaVideo {
    InputMediaPhoto(InputMediaPhoto),
    InputMediaVideo(InputMediaVideo),
}
