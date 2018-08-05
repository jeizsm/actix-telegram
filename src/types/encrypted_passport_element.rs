use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedPassportElement {
    ty: String,
    data: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    files: Option<Vec<PassportFile>>,
    front_side: Option<PassportFile>,
    reverse_side: Option<PassportFile>,
    selfie: Option<PassportFile>,
}