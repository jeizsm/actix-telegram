pub type Integer = i32;

#[derive(Deserialize, Debug)]
pub struct User {
    id: Integer,
    is_bot: bool,
    first_name: String,
    username: String,
}

#[derive(Deserialize, Debug)]
pub struct TelegramResponse {
    ok: bool,
    result: Vec<Update>,
}

#[derive(Deserialize, Debug)]
pub struct Update {
    update_id: Integer,
}
