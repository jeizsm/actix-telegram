use super::super::types::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadStickerFile {
    user_id: Integer,
    png_sticker: InputFile,
}