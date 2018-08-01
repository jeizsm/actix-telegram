extern crate actix_web;
extern crate futures;
extern crate tokio;
#[macro_use] extern crate serde_derive;
extern crate serde;
#[macro_use] extern crate log;

mod methods;
mod types;

use futures::Future;
use std::time::Duration;
use actix_web::{actix::{Actor, Addr, Context, Arbiter, AsyncContext}};
use methods::GetUpdates;

pub struct TelegramBot {
    token: String,
    timeout: i32,
    offset: Option<i32>,
    workers: Option<Vec<Addr<TelegramWorker>>>,
    threads: u8,
}

impl TelegramBot {
    pub fn new(token: String, timeout: i32) -> Self {
        TelegramBot { token, timeout, offset: None, workers: None, threads: 1 }
    }
}

impl Actor for TelegramBot {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("TelegramBot is alive");
        let timeout = Duration::from_secs(self.timeout as u64);

        self.workers = Some((0..self.threads).map(|i| {
            let token = self.token.clone();
            Arbiter::start(|_a| {
                TelegramWorker::new(token)
            })
        }).collect());

        ctx.run_interval(timeout, |actor, ctx| {
            let get_updates = GetUpdates::new(actor.timeout, actor.offset);
            ctx.address().do_send(get_updates);
        });
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramBot is stopped");
    }
}

pub struct TelegramApi {
    token: String,
}

impl TelegramApi {
    fn new(token: String) -> TelegramApi {
        TelegramApi { token }
    }
}

impl Actor for TelegramApi {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("TelegramApi is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramApi is stopped");
    }
}

pub struct TelegramWorker {
    token: String,
}

impl TelegramWorker {
    fn new(token: String) -> TelegramWorker {
        TelegramWorker { token }
    }
}

impl Actor for TelegramWorker {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("TelegramWorker is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramWorker is stopped");
    }
}
