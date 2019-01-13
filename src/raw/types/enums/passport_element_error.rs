use crate::types::*;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum PassportElementError {
    PassportElementErrorDataField(PassportElementErrorDataField),
    PassportElementErrorFrontSide(PassportElementErrorFrontSide),
    PassportElementErrorReverseSide(PassportElementErrorReverseSide),
    PassportElementErrorSelfie(PassportElementErrorSelfie),
    PassportElementErrorFile(PassportElementErrorFile),
    PassportElementErrorFiles(PassportElementErrorFiles),
    PassportElementErrorTranslationFile(PassportElementErrorTranslationFile),
    PassportElementErrorTranslationFiles(PassportElementErrorTranslationFiles),
    PassportElementErrorUnspecified(PassportElementErrorUnspecified),
}
