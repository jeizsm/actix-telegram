mod get_me;
mod get_updates;

use serde::{Serialize, de::DeserializeOwned};
pub use self::get_updates::GetUpdates;
pub use self::get_me::GetMe;
use futures::Future;
use types::TelegramResponse;
use actix_web::{client, HttpMessage};

pub trait TelegramRequest {
    fn send(&self, token: &str) -> Box<Future<Item = TelegramResponse, Error = ()>>;
}

fn send_request<T, R>(token: &str, method: &str, item: &T) -> Box<Future<Item = R, Error = ()>>
where
    R: DeserializeOwned + 'static,
    T: Serialize,
{
    let url = format!("https://api.telegram.org/bot{}/{}", token, method);
    let future = client::post(url)
        .header("User-Agent", "Actix-web")
        .json(item)
        .unwrap()
        .send()
        .map_err(|e| debug!("{:?}", e))
        .and_then(|response| response.json().map_err(|e| debug!("{:?}", e)));
    Box::new(future)
}
