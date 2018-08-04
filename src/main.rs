extern crate actix_telegram;
extern crate actix_web;
extern crate env_logger;
extern crate serde_json;
#[macro_use]
extern crate log;

use actix_telegram::actors::telegram_worker::App;
use actix_telegram::TelegramBot;
use actix_web::actix::{Actor, System};
use std::{env, time::Duration};

fn main() {
    env_logger::init();
    let sys = System::new("example");
    let token = env::var("TELEGRAM_TOKEN").unwrap();
    let app = App::new(|_a| {
        debug!("{}", "test");
        Ok(())
    });

    let _telegram = TelegramBot::new(token, Duration::from_secs(30), app).start();
    sys.run();
}
