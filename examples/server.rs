extern crate actix_telegram;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate log;
extern crate serde_json;

use actix_telegram::{actors::*, methods::*};
use actix_web::actix::{Actor, Arbiter, System};
use futures::future::Future;
use std::env;

fn main() {
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
    let key = Key::new(env::var("KEY").unwrap(), KeyKind::PKCS8);
    let cert = Cert::new(env::var("CERTIFICATE_PEM").unwrap());
    let host = env::var("WEBHOOK_HOST").unwrap();
    let cert_and_key = CertAndKey::new(cert, key);
    let _server = TelegramServer::new("127.0.0.1:59080".to_owned(), token, host, vec![app])
        .set_certificate_and_key(cert_and_key, true)
        .start();
    sys.run();
}
