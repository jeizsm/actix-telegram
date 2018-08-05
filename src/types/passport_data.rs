use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PassportData {
    data: Vec<EncryptedPassportElement>,
    credentials: EncryptedCredentials,
}