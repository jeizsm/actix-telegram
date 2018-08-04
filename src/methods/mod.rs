mod get_me;
mod get_updates;

pub use self::get_me::GetMe;
pub use self::get_updates::GetUpdates;
use actix_web::{client, HttpMessage};
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use types::TelegramResponse;

pub trait TelegramRequest {
    fn send(
        &self,
        token: &str,
        timeout: Duration,
    ) -> Box<Future<Item = TelegramResponse, Error = ()>>;
}

fn send_request<T, R>(
    token: &str,
    method: &str,
    timeout: Duration,
    item: &T,
) -> Box<Future<Item = R, Error = ()>>
where
    R: DeserializeOwned + 'static,
    T: Serialize,
{
    let url = format!("https://api.telegram.org/bot{}/{}", token, method);
    let future = client::post(url)
        .header("User-Agent", "Actix-web")
        .timeout(timeout)
        .json(item)
        .unwrap()
        .send()
        .map_err(|e| error!("request error {:?}", e))
        .and_then(|response| {
            response
                .json()
                .map_err(|e| error!("parsing json error {:?}", e))
        });
    Box::new(future)
}
