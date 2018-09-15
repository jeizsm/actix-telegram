extern crate actix_telegram;
extern crate actix_web;
extern crate env_logger;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate futures;

use actix_telegram::{
    actors::{App, ServerSetWebhook, TelegramApi, TelegramServer},
    methods::*,
    types::*,
};
use actix_web::actix::{Actor, Arbiter, System};
use futures::future::Future;
use std::env;

use actix_web::{
    http::Method,
    server::{HttpServer, Server},
    App as ActixApp, HttpResponse, Json,
};

fn main() {
    println!("{}", std::mem::size_of::<Update>());
    env_logger::init();
    let sys = System::new("example");
    let token = env::var("TELEGRAM_TOKEN").unwrap();
    let app = App::new(|msg, telegram_api| {
        println!("{:?}", msg);
        Arbiter::spawn(
            telegram_api
                .send(GetMe)
                .map(|response| println!("{:?}", response.unwrap()))
                .map_err(|e| {
                    println!("Actor is probably died: {}", e);
                }),
        );
        Ok(())
    });
    let telegram_server = TelegramServer::new("127.0.0.1:59080".to_owned(), token, vec![app])
        .host(env::var("WEBHOOK_HOST").unwrap())
        .start();
    Arbiter::spawn(
        telegram_server
            .send(ServerSetWebhook::new().allowed_updates(vec![AllowedUpdate::EditedMessage]))
            .map(|response| println!("{:?}", response.unwrap()))
            .map_err(|e| {
                println!("Actor is probably died: {}", e);
            }),
    );
    sys.run();
}
