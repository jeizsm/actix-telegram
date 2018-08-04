use std::num::NonZeroU32;

#[derive(Deserialize, Debug)]
pub struct UserId(pub i32);

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

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: UserId,
    pub is_bot: bool,
    pub first_name: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: UpdateId,
}
