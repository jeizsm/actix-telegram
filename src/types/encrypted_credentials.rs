use super::*;

/// Contains data required for decrypting and authenticating EncryptedPassportElement. See the Telegram Passport Documentation for a complete description of the data decryption and authentication processes.
#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedCredentials {
    data: String,
    hash: String,
    secret: String,
}