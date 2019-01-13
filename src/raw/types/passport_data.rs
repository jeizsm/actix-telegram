use crate::types::*;

/// Contains information about Telegram Passport data shared with the bot by the user.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct PassportData {
    /// Array with information about documents and other Telegram Passport elements that was shared with the bot
    pub(crate) data: Vec<EncryptedPassportElement>,
    /// Encrypted credentials required to decrypt the data
    pub(crate) credentials: EncryptedCredentials,
}
