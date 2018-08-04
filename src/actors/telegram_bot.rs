use super::{
    telegram_worker::{App, TelegramWorker},
    TelegramApi,
};
use actix::{Actor, ActorFuture, Addr, Arbiter, AsyncContext, Context, StreamHandler, WrapFuture};
use futures::Stream;
use methods::GetUpdates;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::timer::{self, Interval};
use types::TelegramResponse;

#[derive(Serialize, Debug)]
struct PollUpdates;

pub struct TelegramBot {
    token: String,
    timeout: Duration,
    offset: Option<i32>,
    telegram_api: Option<Addr<TelegramApi>>,
    workers: Vec<Addr<TelegramWorker>>,
    threads: usize,
    apps: Arc<App>,
}

impl TelegramBot {
    pub fn new(token: String, timeout: Duration, apps: App) -> Self {
        TelegramBot {
            token,
            timeout,
            offset: None,
            telegram_api: None,
            workers: Vec::new(),
            threads: 1,
            apps: Arc::new(apps),
        }
    }
}

impl Actor for TelegramBot {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("TelegramBot is alive");

        let telegram_api = TelegramApi::new(self.token.clone(), self.timeout).start();
        let workers = (0..self.threads)
            .map(|_i| {
                let clone = telegram_api.clone();
                let arc = self.apps.clone();
                Arbiter::start(move |_a| TelegramWorker::new(clone, arc))
            })
            .collect();
        self.workers = workers;
        self.telegram_api = Some(telegram_api);

        ctx.set_mailbox_capacity(1);
        let stream = Interval::new(Instant::now(), Duration::from_secs(1)).map(|_| PollUpdates);
        ctx.add_stream(stream);
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramBot is stopped");
    }
}

impl StreamHandler<PollUpdates, timer::Error> for TelegramBot {
    fn handle(&mut self, _msg: PollUpdates, ctx: &mut Context<Self>) {
        let timeout = self.timeout.as_secs() as u16 - 1;
        let msg = GetUpdates::new(timeout, self.offset);
        debug!("TelegramBot.GetUpdates {:?}", msg);

        let telegram_api = self.telegram_api.as_ref().unwrap();
        let actor_future = telegram_api
            .send(msg)
            .into_actor(self)
            .map(|response: Result<TelegramResponse, ()>, actor, _ctx| {
                let response = response.unwrap();
                debug!("response received {:?}", response);
                actor.offset = response.result.last().map(|i| i.update_id + 1);
                for (i, result) in response.result.into_iter().enumerate() {
                    actor.workers[i % actor.threads].do_send(result);
                }
            })
            .map_err(|e, _actor, _ctx| debug!("mailbox error {:?}", e));
        ctx.wait(actor_future);
    }
}
