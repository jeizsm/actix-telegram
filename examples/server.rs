extern crate actix_telegram;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate log;
extern crate serde_json;

use actix_telegram::actors::{telegram_server::*, TelegramApi};
use actix_telegram::application::UpdateHandler;
use actix_telegram::types::update::{MessageUpdate, Update, UpdateKind};
use actix_telegram::types::{Message, ChatIdOrUsername};
use actix_telegram::methods::{GetMe, SendMessage};
use actix_web::actix::{Actor, Addr, System};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use futures::Future;

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
        let clone = arc.clone();
        Test { state: clone }
    })
    .set_workers(4)
    .set_certificate_and_key(cert_and_key, true)
    .start();
    sys.run();
}

struct Test {
    state: Arc<Mutex<HashMap<i64, ()>>>,
}

struct Resource<'a, U, S> {
    value: U,
    state: &'a S,
    telegram_api: &'a Addr<TelegramApi>,
}

impl<'a, V, S> Resource<'a, V, S> {
    fn map<M, F>(self, function: F) -> Resource<'a, M, S>
    where
        F: FnOnce(V) -> M,
    {
        Resource {
            value: function(self.value),
            state: self.state,
            telegram_api: self.telegram_api,
        }
    }

    fn f<F>(self, function: F) -> bool
    where
        F: FnOnce(V, &'a Addr<TelegramApi>, &'a S) -> bool,
    {
        function(self.value, self.telegram_api, self.state)
    }
}

impl<'a, S> Resource<'a, Option<Message>, S> {
    fn command<F>(self, starts_with: &str, function: F) -> bool
    where
        F: FnOnce(Message, &'a Addr<TelegramApi>, &'a S) -> bool,
    {
        if let Some(message) = self.value {
            if message
                .text()
                .as_ref()
                .map_or(false, |text| text.starts_with(starts_with))
            {
                function(message, self.telegram_api, self.state)
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<'a, S> Resource<'a, Option<&'a Message>, S> {
    fn command<F>(self, starts_with: &str, function: F) -> bool
    where
        F: FnOnce(&'a Message, &'a Addr<TelegramApi>, &'a S) -> bool,
    {
        if let Some(message) = self.value {
            if message
                .text()
                .as_ref()
                .map_or(false, |text| text.starts_with(starts_with))
            {
                function(message, self.telegram_api, self.state)
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl UpdateHandler for Test {
    fn handle(&self, update: Update, telegram_api: &Addr<TelegramApi>) -> Result<(), Update> {
        let res = {
            let resource = Resource {
                value: &update,
                state: &self.state,
                telegram_api: telegram_api,
            };
            resource
                .map(|update| {
                    if let UpdateKind::Message(message) = update.kind() {
                        Some(message)
                    } else {
                        None
                    }
                })
                .f(|message, telegram_api, _state| {
                    let id = *message.unwrap().chat().id();
                    let chat_id = ChatIdOrUsername::Id(id);
                    let message_id = *message.unwrap().message_id();
                    let clone = telegram_api.clone();
                    actix::spawn(
                        telegram_api
                            .send(GetMe)
                            .map(move |response| {
                                let mut send_message = SendMessage::new(chat_id, response.unwrap().first_name().clone());
                                send_message.set_reply_to_message_id(Some(message_id));
                                // println!("send message {:?}", response)
                                actix::spawn(
                                    clone
                                        .send(send_message)
                                        .map(|response| println!("send message {:?}", response))
                                        .map_err(|e| println!("Actor is probably died: {}", e))
                                )
                            })
                            .map_err(|e| println!("Actor is probably died: {}", e)),
                    );
                    let mut hash_map = self.state.lock().unwrap();
                    hash_map
                        .entry(*message.unwrap().from().as_ref().unwrap().id())
                        .or_insert(());
                    true
                })
        };
        println!("{:?}", self.state);
        if res {
            Ok(())
        } else {
            Err(update)
        }
    }
}
