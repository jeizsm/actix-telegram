use super::{TelegramApi, TelegramWorker};
use crate::application::UpdateHandler;
use crate::methods::OptimizedGetUpdates;
use crate::types::UpdateId;
use actix::{Actor, ActorFuture, Addr, Arbiter, AsyncContext, Context, StreamHandler, WrapFuture};
use futures::Stream;
use std::time::{Duration, Instant};
use tokio::timer::{self, Interval};

struct PollUpdates;

pub struct TelegramBot<F, H>
where
    H: UpdateHandler + 'static,
    F: Fn() -> H + Send + Clone + 'static,
{
    token: String,
    timeout: Duration,
    offset: Option<UpdateId>,
    telegram_api: Option<Addr<TelegramApi>>,
    workers: Vec<Addr<TelegramWorker<H>>>,
    threads: usize,
    factory: F,
}

impl<F, H> TelegramBot<F, H>
where
    H: UpdateHandler + 'static,
    F: Fn() -> H + Send + Clone + 'static,
{
    pub fn new(token: String, timeout: u16, factory: F) -> Self {
        let timeout = Duration::from_secs(u64::from(timeout));
        Self {
            token,
            timeout,
            offset: None,
            telegram_api: None,
            workers: Vec::new(),
            threads: 1,
            factory,
        }
    }

    pub fn set_workers(mut self, num: usize) -> Self {
        self.threads = num;
        self
    }
}

impl<F, H> Actor for TelegramBot<F, H>
where
    H: UpdateHandler + 'static,
    F: Fn() -> H + Send + Clone + 'static,
{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("TelegramBot is alive");

        let telegram_api =
            TelegramApi::new(self.token.clone(), self.timeout.as_secs() as u16).start();
        let workers = (0..self.threads)
            .map(|_i| {
                let clone = telegram_api.clone();
                let apps = self.factory.clone();
                Arbiter::start(move |_a| {
                    let apps = (apps)();
                    TelegramWorker::new(clone, apps)
                })
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

impl<F, H> StreamHandler<PollUpdates, timer::Error> for TelegramBot<F, H>
where
    H: UpdateHandler + 'static,
    F: Fn() -> H + Send + Clone + 'static,
{
    fn handle(&mut self, _: PollUpdates, ctx: &mut Context<Self>) {
        let timeout = self.timeout.as_secs() as u16;
        let mut msg = OptimizedGetUpdates::new();
        msg.set_offset(self.offset).set_timeout(Some(timeout));
        debug!("TelegramBot.GetUpdates {:?}", msg);

        let telegram_api = self.telegram_api.as_ref().unwrap();
        let actor_future = telegram_api
            .send(msg)
            .into_actor(self)
            .map(|response, actor, _ctx| {
                let _ = response
                    .map(|response| {
                        debug!("response received {:?}", response);
                        actor.offset = response.last().map(|i| (i.update_id().get() + 1).into());
                        for (i, result) in response.into_iter().enumerate() {
                            actor.workers[i % actor.threads].do_send(result);
                        }
                    })
                    .map_err(|e| error!("response error {:?}", e));
            })
            .map_err(|e, _actor, _ctx| error!("mailbox error {:?}", e));
        ctx.wait(actor_future);
    }
}
