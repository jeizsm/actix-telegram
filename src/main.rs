extern crate actix_telegram;
extern crate actix_web;
extern crate env_logger;
extern crate serde_json;

use actix_telegram::TelegramBot;
use actix_web::actix::{Actor, System};
use std::{env, time::Duration};

fn main() {
    env_logger::init();
    let sys = System::new("example");
    let token = env::var("TELEGRAM_TOKEN").unwrap();
    let _telegram = TelegramBot::new(token, Duration::from_secs(5)).start();
    sys.run();
}
