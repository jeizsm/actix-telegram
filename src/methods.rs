use std::env;
use futures::Future;
use actix_web::{actix::*, client, HttpMessage};
use super::Telegram;
use types::TelegramResponse;

pub trait TelegramRequest {
    fn send(&self) -> Box<Future<Item = TelegramResponse, Error = ()>>;
}

/// Define message
pub struct GetMe;

impl Message for GetMe {
    type Result = Result<TelegramResponse, ()>;
}

impl TelegramRequest for GetMe {
    fn send(&self) -> Box<Future<Item = TelegramResponse, Error = ()>> {
        let token = env::var("TELEGRAM_TOKEN").unwrap();
        let method = "getMe";
        let url = format!("https://api.telegram.org/bot{}/{}", token, method);
        let a = client::get(url)   // <- Create request builder
                .header("User-Agent", "Actix-web")
                .finish().unwrap()
                .send()                               // <- Send http request
                .map_err(|_| ())
                .and_then(|response| {                // <- server http response
                    response
                        .json()
                        .map_err(|_| ())
                });
        Box::new(a)
    }
}

/// Define handler for `GetMe` message
impl<T> Handler<T> for Telegram
    where T: TelegramRequest + Message<Result = Result<TelegramResponse, ()>> {
    type Result = Box<Future<Item = TelegramResponse, Error = ()>>;

    fn handle(&mut self, msg: T, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");

        msg.send()
    }
}

impl StreamHandler<GetMe, ()> for Telegram {
    fn handle(&mut self, item: GetMe, ctx: &mut Context<Telegram>) {
        println!("PING");
        ctx.spawn(
            item.send().and_then(|body| {
                println!("Response: {:?}", body);
                Ok(())
            }).into_actor(self)
        );
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        println!("finished");
    }
}
