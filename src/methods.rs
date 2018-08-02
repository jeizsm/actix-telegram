use actix::{ActorFuture, AsyncContext, Context, Handler, Message, StreamHandler, WrapFuture};
use actix_web::{client, HttpMessage};
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};
use tokio::timer::Error;
use types::{Integer, TelegramResponse};
use TelegramApi;
use TelegramBot;

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

pub trait TelegramRequest {
    fn send(&self, token: &str) -> Box<Future<Item = TelegramResponse, Error = ()>>;
}

#[derive(Serialize, Debug)]
pub struct GetMe;

impl Message for GetMe {
    type Result = Result<TelegramResponse, ()>;
}

impl TelegramRequest for GetMe {
    fn send(&self, token: &str) -> Box<Future<Item = TelegramResponse, Error = ()>> {
        send_request(token, "getMe", self)
    }
}

#[derive(Serialize, Debug)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

impl GetUpdates {
    pub fn new(timeout: Integer, offset: Option<Integer>) -> Self {
        GetUpdates {
            offset,
            timeout: Some(timeout),
            allowed_updates: None,
            limit: None,
        }
    }
}

impl Message for GetUpdates {
    type Result = Result<TelegramResponse, ()>;
}

impl TelegramRequest for GetUpdates {
    fn send(&self, token: &str) -> Box<Future<Item = TelegramResponse, Error = ()>> {
        send_request(token, "getUpdates", self)
    }
}

#[derive(Serialize, Debug)]
pub struct PollUpdates;

impl StreamHandler<PollUpdates, Error> for TelegramBot {
    fn handle(&mut self, _msg: PollUpdates, ctx: &mut Context<Self>) {
        debug!("TelegramBot.GetUpdates received");
        let msg = GetUpdates::new(self.timeout, self.offset);
        debug!("{:?}", msg);

        let future = self.client
            .json(msg)
            .unwrap()
            .send()
            .map_err(|e| debug!("{:?}", e))
            .and_then(|response| response.json().map_err(|e| debug!("{:?}", e)));
        let actor_future = future.into_actor(self).map(
            |response: TelegramResponse, actor, _ctx| {
                debug!("{:?}", response);
                actor.offset = response.result.last().map(|i| i.update_id + 1);
            },
        );
        ctx.wait(actor_future);
    }
}
