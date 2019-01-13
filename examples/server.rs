extern crate actix_telegram;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate log;
extern crate serde_json;

use actix_telegram::actors::{telegram_server::*, TelegramApi};
use actix_telegram::methods::{GetMe, SendMessage};
use actix_telegram::types::Message;
use actix_telegram::App;
use actix_web::actix::{Actor, Addr, System};
use futures::Future;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

fn main() {
    env_logger::init();
    let sys = System::new("example");
    let token = env::var("TELEGRAM_TOKEN").unwrap();
    let key = Key::new(env::var("KEY").unwrap(), KeyKind::PKCS8);
    let cert = Cert::new(env::var("CERTIFICATE_PEM").unwrap());
    let host = env::var("WEBHOOK_HOST").unwrap();
    let cert_and_key = CertAndKey::new(cert, key);
    let arc = Arc::new(Mutex::new(HashMap::new()));
    let _server = TelegramServer::new("127.0.0.1:59080".to_owned(), token, host, move || {
        App::new(
            |resource| resource.message().command(handle, "/new"),
            arc.clone(),
        )
    })
    .set_workers(4)
    .set_certificate_and_key(cert_and_key, true)
    .start();
    sys.run();
}

fn handle(
    message: &Message,
    telegram_api: &Addr<TelegramApi>,
    state: &Arc<Mutex<HashMap<i64, ()>>>,
) -> bool {
    let chat_id = *message.chat().id();
    let message_id = *message.message_id();
    let clone = telegram_api.clone();
    actix::spawn(
        telegram_api
            .send(GetMe)
            .map(move |response| {
                let mut send_message =
                    SendMessage::new(chat_id, response.unwrap().first_name().clone());
                send_message.set_reply_to_message_id(Some(message_id));
                actix::spawn(
                    clone
                        .send(send_message)
                        .map(|response| println!("send message {:?}", response))
                        .map_err(|e| println!("Actor is probably died: {}", e)),
                )
            })
            .map_err(|e| println!("Actor is probably died: {}", e)),
    );
    let mut hash_map = state.lock().unwrap();
    hash_map
        .entry(*message.from().as_ref().unwrap().id())
        .or_insert(());
    true
}
