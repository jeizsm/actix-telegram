use crate::types::*;

/// Use this method to upload a .png file with a sticker for later use in createNewStickerSet and addStickerToSet methods (can be used multiple times). Returns the uploaded File on success.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "File"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct UploadStickerFile {
    /// User identifier of sticker file owner
    pub(crate) user_id: Integer,
    /// Png image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. More info on Sending Files »
    pub(crate) png_sticker: InputFile,
}
