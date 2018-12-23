extern crate actix_telegram;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate log;
extern crate serde_json;

use actix_telegram::actors::{telegram_server::*, App, TelegramApi};
use actix_telegram::{methods::*, types::*};
use actix_web::actix::{Actor, Addr, System};
use futures::Future;
use std::env;

fn main() {
    env_logger::init();
    let sys = System::new("example");
    let token = env::var("TELEGRAM_TOKEN").unwrap();
    let api = TelegramApi::new(token, 10).start();
    let user_id: i64 = env::var("USER_ID").unwrap().parse().unwrap();
    let chat_id = ChatIdOrUsername::Id(env::var("CHAT_ID").unwrap().parse().unwrap());
    let get_chat_member = GetChatMember::new(chat_id, user_id);
    actix::spawn(
        api.send(get_chat_member)
            .map(|a| {
                println!("{:?}", a);
                actix::System::current().stop();
                ()
            })
            .map_err(|_| ()),
    );
    sys.run();
}
