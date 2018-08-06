use super::*;

/// Contains information about Telegram Passport data shared with the bot by the user.
#[derive(Serialize, Deserialize, Debug)]
pub struct PassportData {
    data: Vec<EncryptedPassportElement>,
    credentials: EncryptedCredentials,
}