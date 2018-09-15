extern crate actix_telegram;
extern crate actix_web;
extern crate env_logger;
extern crate serde_json;
extern crate log;
extern crate futures;

use actix_telegram::{methods::GetMe, App, TelegramBot};
use actix_web::actix::{Actor, Arbiter, System};
use futures::future::Future;
use std::env;

fn main() {
    env_logger::init();
    let sys = System::new("example");
    let token = env::var("TELEGRAM_TOKEN").unwrap();
    let second_app = App::new(|msg, _| {
        println!("{}", "next");
        Err(msg)
    });
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

    let _telegram = TelegramBot::new(token, 30, vec![second_app, app]).start();
    sys.run();
}
