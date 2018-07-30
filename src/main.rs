extern crate actix_web;
extern crate actix_telegram;

use actix_telegram::Telegram;
use actix_web::{actix::{System, Actor}};

fn main() {
    let sys = System::new("example");
    let telegram = Telegram.start();

    sys.run();
}
