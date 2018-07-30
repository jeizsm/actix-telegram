extern crate actix_web;
extern crate futures;
#[macro_use] extern crate serde_derive;

use futures::future::Future;
use actix_web::{actix::*, client, HttpMessage};
use std::env;

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

fn get_me() -> Box<Future<Item = TelegramResponse, Error = ()>> {
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

/// Define message
struct GetMe;

impl Message for GetMe {
    type Result = Result<TelegramResponse, ()>;
}

// Define actor
struct Telegram;

// Provide Actor implementation for our actor
impl Actor for Telegram {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}


/// Define handler for `Ping` message
impl Handler<GetMe> for Telegram {
    type Result = Box<Future<Item = TelegramResponse, Error = ()>>;

    fn handle(&mut self, msg: GetMe, ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");

        get_me()
    }
}

fn main() {
    let sys = System::new("example");
    let telegram = Telegram.start();
    let result = telegram.send(GetMe);

    Arbiter::spawn(
        result.map(|res| {
            match res {
                Ok(body) => println!("Response: {:?}", body),
                Err(_) => println!("{}", "error"),
            }
        })
        .map_err(|e| {
            println!("Actor is probably died: {}", e);
        })
    );
    sys.run();
}
