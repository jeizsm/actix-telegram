extern crate actix_telegram;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate log;
extern crate serde_json;

use actix_telegram::actors::{telegram_server::*, App, TelegramApi};
use actix_telegram::types::update::Update;
use actix_web::actix::{Actor, Addr, System};
use std::env;

fn main() {
    env_logger::init();
    let sys = System::new("example");
    let token = env::var("TELEGRAM_TOKEN").unwrap();
    let print = App::new(print_update);
    let key = Key::new(env::var("KEY").unwrap(), KeyKind::PKCS8);
    let cert = Cert::new(env::var("CERTIFICATE_PEM").unwrap());
    let host = env::var("WEBHOOK_HOST").unwrap();
    let cert_and_key = CertAndKey::new(cert, key);
    let _server = TelegramServer::new("127.0.0.1:59080".to_owned(), token, host, vec![print])
        .set_certificate_and_key(cert_and_key, true)
        .start();
    sys.run();
}

fn print_update(update: Update, _: &Addr<TelegramApi>) -> Result<(), Update> {
    println!("{:?}", update);
    Ok(())
}
