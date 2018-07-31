extern crate actix_web;
extern crate futures;
extern crate tokio;
#[macro_use] extern crate serde_derive;

use std::time::Duration;
use std::time::Instant;
use futures::{Stream, future::Future};
use actix_web::{actix::*, client, HttpMessage};
use std::env;
use tokio::timer::Interval;

#[derive(Deserialize, Debug)]
struct User {
    id: i64,
    is_bot: bool,
    first_name: String,
    username: String,
}

#[derive(Deserialize, Debug)]
struct TelegramResponse {
    ok: bool,
    result: User,
}

/// Define message
struct GetMe;

impl Message for GetMe {
    type Result = Result<TelegramResponse, ()>;
}

impl GetMe {
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

// Define actor
pub struct Telegram;

// Provide Actor implementation for our actor
impl Actor for Telegram {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
        let stream = Interval::new(Instant::now(), Duration::from_secs(10))
            .map(|_| GetMe)
            .map_err(|e| panic!("interval errored; err={:?}", e));
        Self::add_stream(stream, ctx);
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

/// Define handler for `GetMe` message
impl Handler<GetMe> for Telegram {
    type Result = Box<Future<Item = TelegramResponse, Error = ()>>;

    fn handle(&mut self, msg: GetMe, ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");

        msg.send()
    }
}

impl StreamHandler<GetMe, ()> for Telegram {
    fn handle(&mut self, item: GetMe, ctx: &mut Context<Telegram>) {
        println!("PING");
        Arbiter::spawn(
            item.send().and_then(|body| {
                println!("Response: {:?}", body);
                Ok(())
            })
        );
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        println!("finished");
    }
}
