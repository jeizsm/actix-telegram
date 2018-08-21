use serde::ser::{Serialize, Serializer};
use std::fmt::{self, Debug, Formatter};
use std::io::Read;
use std::num::NonZeroU32;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, NewType)]
pub struct UserId(i32);

#[derive(Serialize, Deserialize, Debug, NewType)]
pub struct ChatId(i64);

#[derive(Serialize, Deserialize, Debug, NewType)]
pub struct MessageId(i32);

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct UpdateId(NonZeroU32);

impl UpdateId {
    pub fn new(update_id: u32) -> Self {
        unsafe { UpdateId(NonZeroU32::new_unchecked(update_id)) }
    }

    pub fn get(self) -> u32 {
        self.0.get()
    }
}

#[derive(Deserialize, Debug)]
pub struct TelegramResponse<T> {
    pub ok: bool,
    pub description: Option<String>,
    pub result: Option<T>,
}

pub enum InputFile {
    Memory {
        name: String,
        source: Box<Read + Send>,
        len: Option<u64>,
    },
    Disk {
        path: String,
    },
}

impl Serialize for InputFile {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            InputFile::Memory { name, .. } => {
                let attach = format!("attach://{}", name);
                serializer.serialize_str(&attach)
            }
            InputFile::Disk { path } => {
                let path: &Path = path.as_ref();
                let field_name = path.file_name().unwrap().to_str().unwrap();
                let attach = format!("attach://{}", field_name);
                serializer.serialize_str(&attach)
            }
        }
    }
}

impl Debug for InputFile {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            InputFile::Disk { path } => write!(f, "InputFile {{ path: {} }}", path),
            InputFile::Memory { name, len, .. } => {
                write!(f, "InputFile {{ name: {}, len: {:?} }}", name, len)
            }
        }
    }
}
