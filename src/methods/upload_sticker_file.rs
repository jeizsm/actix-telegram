use super::super::types::*;

/// Use this method to upload a .png file with a sticker for later use in createNewStickerSet and addStickerToSet methods (can be used multiple times). Returns the uploaded File on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct UploadStickerFile {
    user_id: Integer,
    png_sticker: InputFile,
}