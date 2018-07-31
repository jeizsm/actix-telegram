#[derive(Deserialize, Debug)]
struct User {
    id: i64,
    is_bot: bool,
    first_name: String,
    username: String,
}

#[derive(Deserialize, Debug)]
pub struct TelegramResponse {
    ok: bool,
    result: User,
}
