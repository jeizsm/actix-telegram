pub type Integer = i32;

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: Integer,
    pub is_bot: bool,
    pub first_name: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct TelegramResponse {
    pub ok: bool,
    pub result: Vec<Update>,
}

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: Integer,
}
