use crate::types::*;

/// Use this method to add a new sticker to a set created by the bot. Returns True on success.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "True"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct AddStickerToSet {
    /// User identifier of sticker set owner
    pub(crate) user_id: Integer,
    /// Sticker set name
    pub(crate) name: String,
    /// Png image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    pub(crate) png_sticker: InputFileOrString,
    /// One or more emoji corresponding to the sticker
    pub(crate) emojis: String,
    /// A JSON-serialized object for position where the mask should be placed on faces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mask_position: Option<MaskPosition>,
}
