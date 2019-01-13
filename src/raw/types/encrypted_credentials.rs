use crate::types::*;

/// Contains data required for decrypting and authenticating EncryptedPassportElement. See the Telegram Passport Documentation for a complete description of the data decryption and authentication processes.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON-serialized data with unique user's payload, data hashes and secrets required for EncryptedPassportElement decryption and authentication
    pub(crate) data: String,
    /// Base64-encoded data hash for data authentication
    pub(crate) hash: String,
    /// Base64-encoded secret, encrypted with the bot's public RSA key, required for data decryption
    pub(crate) secret: String,
}
