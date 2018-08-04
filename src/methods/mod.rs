mod get_me;
mod get_updates;

pub use self::get_me::GetMe;
pub use self::get_updates::GetUpdates;
use actix_web::{client, HttpMessage};
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use types::TelegramResponse;

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
                .then(|response: Result<TelegramResponse<R>, _>| match response {
                    Ok(response) => {
                        if response.ok {
                            Ok(response.result.unwrap())
                        } else {
                            error!("telegram error {:?}", response.description);
                            Err(())
                        }
                    }
                    Err(e) => {
                        error!("parsing json error {:?}", e);
                        Err(())
                    }
                })
        });
    Box::new(future)
}
