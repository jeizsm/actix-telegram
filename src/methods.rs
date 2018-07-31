use futures::Future;
use actix_web::{actix::*, client, HttpMessage};
use super::Telegram;
use types::TelegramResponse;
use serde::{Serialize, de::DeserializeOwned};

pub trait TelegramRequest {
    fn send(self, token: String) -> Box<Future<Item = TelegramResponse, Error = ()>>;
}

/// Define message
#[derive(Serialize, Debug)]
pub struct GetMe;

impl Message for GetMe {
    type Result = Result<TelegramResponse, ()>;
}

fn send_request<T, R>(token: String, method: String, item: T) -> Box<Future<Item = R, Error = ()>>
    where R: DeserializeOwned + 'static,
    T: Serialize {
    let url = format!("https://api.telegram.org/bot{}/{}", token, method);
    let future = client::post(url)
            .header("User-Agent", "Actix-web")
            .json(item).unwrap()
            .send()
            .map_err(|_| ())
            .and_then(|response| {
                response
                    .json()
                    .map_err(|_| ())
            });
    Box::new(future)
}

impl TelegramRequest for GetMe {
    fn send(self, token: String) -> Box<Future<Item = TelegramResponse, Error = ()>> {
        send_request(token, "getMe".to_string(), self)
    }
}

/// Define handler for `GetMe` message
impl<T> Handler<T> for Telegram
    where T: TelegramRequest + Message<Result = Result<TelegramResponse, ()>> {
    type Result = Box<Future<Item = TelegramResponse, Error = ()>>;

    fn handle(&mut self, msg: T, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");

        msg.send(self.token.clone())
    }
}

impl StreamHandler<GetMe, ()> for Telegram {
    fn handle(&mut self, item: GetMe, ctx: &mut Context<Telegram>) {
        println!("PING");
        ctx.spawn(
            item.send(self.token.clone()).and_then(|body| {
                println!("Response: {:?}", body);
                Ok(())
            }).into_actor(self)
        );
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        println!("finished");
    }
}
