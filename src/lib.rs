extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate tokio;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate log;

mod methods;
mod types;

use actix::{Actor, Addr, Arbiter, AsyncContext, Context};
use actix_web::client::{self, ClientRequestBuilder};
use futures::Stream;
use methods::PollUpdates;
use std::time::{Duration, Instant};
use tokio::timer::Interval;

pub struct TelegramBot {
    token: String,
    timeout: i32,
    offset: Option<i32>,
    client: ClientRequestBuilder,
    workers: Option<Vec<Addr<TelegramWorker>>>,
    threads: u8,
}

impl TelegramBot {
    pub fn new(token: String, timeout: i32) -> Self {
        let url = format!("https://api.telegram.org/bot{}/getUpdates", token);
        let mut client = client::post(url);
        client
            .header("User-Agent", "actix-web")
            .timeout(Duration::from_secs(timeout as u64 + 1));

        TelegramBot {
            token,
            timeout,
            offset: None,
            client,
            workers: None,
            threads: 1,
        }
    }
}

impl Actor for TelegramBot {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("TelegramBot is alive");
        let timeout = Duration::from_secs(self.timeout as u64);

        let workers = (0..self.threads)
            .map(|_i| {
                let token = self.token.clone();
                Arbiter::start(|_a| TelegramWorker::new(token))
            })
            .collect();
        self.workers = Some(workers);

        let stream = Interval::new(Instant::now(), timeout).map(|_| PollUpdates);
        ctx.add_stream(stream);
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

    fn started(&mut self, _ctx: &mut Context<Self>) {
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

    fn started(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramWorker is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramWorker is stopped");
    }
}
