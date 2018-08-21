use actix::{Actor, Context};
use actix_web::{
    client::{self, MultipartForm},
    HttpMessage,
};
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use types::TelegramResponse;

pub struct TelegramApi {
    pub(crate) token: String,
    pub(crate) timeout: Duration,
}

impl TelegramApi {
    pub fn new(token: String, timeout: u16) -> TelegramApi {
        let timeout = Duration::from_secs(u64::from(timeout));
        TelegramApi { token, timeout }
    }

    pub fn send_request<T, R>(
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

    pub fn send_multipart_request<R>(
        token: &str,
        method: &str,
        timeout: Duration,
        item: MultipartForm,
    ) -> Box<Future<Item = R, Error = ()>>
    where
        R: DeserializeOwned + 'static,
    {
        let url = format!("https://api.telegram.org/bot{}/{}", token, method);
        let future = client::post(url)
            .timeout(timeout)
            .multipart(item)
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
