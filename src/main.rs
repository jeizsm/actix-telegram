extern crate actix_web;
extern crate actix_telegram;

use actix_telegram::TelegramBot;
use actix_web::{actix::{System, Actor}};
use std::env;

fn main() {
    let sys = System::new("example");
    let _token = env::var("TELEGRAM_TOKEN").unwrap();
    let _telegram = TelegramBot::new(1).start();

    sys.run();
}
