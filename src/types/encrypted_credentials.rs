use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedCredentials {
    data: String,
    hash: String,
    secret: String,
}