extern crate actix_telegram;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate log;
extern crate serde_json;

use actix_telegram::application::UpdateHandler;
use actix_telegram::methods::{DeleteWebhook, SendMessage};
use actix_telegram::types::update::{Update, UpdateKind};
use actix_telegram::types::ChatIdOrUsername;
use actix_telegram::{App, TelegramApi, TelegramBot};
use actix_web::actix::{self, Actor, Addr, System};
use futures::future::Future;
use std::env;

fn main() {
    env_logger::init();
    let sys = System::new("example");
    let token = env::var("TELEGRAM_TOKEN").unwrap();
    let api = TelegramApi::new(token.clone(), 10).start();
    actix::spawn(
        api.send(DeleteWebhook)
            .map(|response| println!("removed webhook {:?}", response))
            .map_err(|e| println!("Actor is probably died: {}", e)),
    );
    let print_update = App::new(print_update);
    let greeter = App::new(greet);
    let _telegram = TelegramBot::new(token, 30, || Test).start();
    sys.run();
}

fn print_update(update: Update, _: &Addr<TelegramApi>) -> Result<(), Update> {
    println!("{:?}", update);
    Ok(())
}

struct Test;

impl UpdateHandler for Test {
    fn handle(&self, update: Update, telegram_api: &Addr<TelegramApi>) -> Result<(), Update> {
        if let UpdateKind::Message(message) = update.kind() {
            if let Some(ref members) = message.new_chat_members() {
                println!("{:?}", members);
                let member = members.first().unwrap();
                if !member.is_bot() {
                    let chat_id = ChatIdOrUsername::Id(*message.chat().id());
                    let mut send_message = SendMessage::new(chat_id, "Welcome");
                    send_message.set_reply_to_message_id(Some(*message.message_id()));
                    actix::spawn(
                        telegram_api
                            .send(send_message)
                            .map(|response| println!("send message {:?}", response))
                            .map_err(|e| println!("Actor is probably died: {}", e)),
                    )
                }
                return Ok(());
            }
        }
        Err(update)
    }
}

fn greet(update: Update, telegram_api: &Addr<TelegramApi>) -> Result<(), Update> {
    if let UpdateKind::Message(message) = update.kind() {
        if let Some(ref members) = message.new_chat_members() {
            println!("{:?}", members);
            let member = members.first().unwrap();
            if !member.is_bot() {
                let chat_id = ChatIdOrUsername::Id(*message.chat().id());
                let mut send_message = SendMessage::new(chat_id, "Welcome");
                send_message.set_reply_to_message_id(Some(*message.message_id()));
                actix::spawn(
                    telegram_api
                        .send(send_message)
                        .map(|response| println!("send message {:?}", response))
                        .map_err(|e| println!("Actor is probably died: {}", e)),
                )
            }
            return Ok(());
        }
    }
    Err(update)
}
