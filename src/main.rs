extern crate actix_web;
extern crate actix_telegram;

use actix_telegram::Telegram;
use actix_web::{actix::{System, Actor}};
use std::env;

fn main() {
    let sys = System::new("example");
    let token = env::var("TELEGRAM_TOKEN").unwrap();
    let _telegram = Telegram::new(token).start();

    sys.run();
}
