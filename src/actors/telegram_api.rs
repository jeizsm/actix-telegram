use crate::types::TelegramResponse;
use actix::{Actor, Context};
use actix_web::{client, HttpMessage};
use futures::Future;
use multipart_rfc7578::{Form, SetBody};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::time::Duration;
use failure::Error;

pub struct TelegramApi {
    pub(crate) token: String,
    pub(crate) timeout: Duration,
}

impl TelegramApi {
    #[inline]
    pub fn new(token: String, timeout: u16) -> Self {
        let timeout = Duration::from_secs(u64::from(timeout));
        Self { token, timeout }
    }

    pub fn send_request<T, R>(
        token: &str,
        method: &str,
        timeout: Duration,
        item: &T,
    ) -> Box<Future<Item = R, Error = Error>>
    where
        R: DeserializeOwned + Debug + 'static,
        T: Serialize,
    {
        let url = format!("https://api.telegram.org/bot{}/{}", token, method);
        let future = client::post(url)
            .timeout(timeout)
            .json(item)
            .unwrap()
            .send()
            .from_err()
            .and_then(|response| {
                response
                    .json()
                    .from_err()
                    .and_then(|response: TelegramResponse<R>| {
                        debug!("response {:?}", response);
                        if response.ok {
                            Ok(response.result.unwrap())
                        } else {
                            bail!(response.description.unwrap_or_else(|| "no description".into()))
                        }
                    })
            }).map_err(|err| {
                error!("send request error: {}", err);
                err
            });
        Box::new(future)
    }

    pub fn send_multipart_request<R>(
        token: &str,
        method: &str,
        timeout: Duration,
        item: Form,
    ) -> Box<Future<Item = R, Error = Error>>
    where
        R: DeserializeOwned + Debug + 'static,
    {
        let url = format!("https://api.telegram.org/bot{}/{}", token, method);
        let mut request = client::post(url);
        let future = item
            .set_body(request.timeout(timeout))
            .unwrap()
            .send()
            .from_err()
            .and_then(|response| {
                response
                    .json()
                    .from_err()
                    .and_then(|response: TelegramResponse<R>| {
                        debug!("response {:?}", response);
                        if response.ok {
                            Ok(response.result.unwrap())
                        } else {
                            bail!(response.description.unwrap_or_else(|| "no description".into()))
                        }
                    })
            }).map_err(|err| {
                error!("send request error: {}", err);
                err
            });
        Box::new(future)
    }
}

impl Actor for TelegramApi {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramApi is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramApi is stopped");
    }
}
