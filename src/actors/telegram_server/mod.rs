#[cfg(feature = "tls-server")]
mod tls_server;
mod types;

use self::types::{OptionFlags, ReqState};
use super::TelegramApi;
use crate::application::{IntoUpdateHandler, UpdateHandler};
use crate::methods::SetWebhook;
use crate::types::{InputFile, True, Update};
use actix::{Actor, Addr, Context, Handler};
use actix_net::server::Server;
use actix_web::dev::HttpResponseBuilder;
use actix_web::{http::Method, server::HttpServer, App as ActixApp, HttpResponse, Json, State};
use futures::Future;

#[cfg(feature = "tls-server")]
pub use self::tls_server::*;

pub struct TelegramServer<F, H, UH>
where
    H: IntoUpdateHandler<Handler = UH> + 'static,
    UH: UpdateHandler + 'static,
    F: Fn() -> H + Send + Clone + 'static,
{
    addr: String,
    host: String,
    url: Option<String>,
    token: String,
    threads: usize,
    factory: F,
    server: Option<Addr<Server>>,
    options: OptionFlags,
    #[cfg(feature = "tls-server")]
    cert_and_key: Option<CertAndKey>,
}

impl<F, H, UH> TelegramServer<F, H, UH>
where
    H: IntoUpdateHandler<Handler = UH> + 'static,
    UH: UpdateHandler + 'static,
    F: Fn() -> H + Send + Clone + 'static,
{
    pub fn new(addr: String, token: String, host: String, factory: F) -> Self {
        Self {
            addr,
            host,
            url: None,
            threads: 1,
            factory,
            server: None,
            token,
            #[cfg(feature = "tls-server")]
            cert_and_key: None,
            options: OptionFlags::default(),
        }
    }

    pub fn set_workers(mut self, num: usize) -> Self {
        self.threads = num;
        self
    }

    pub fn set_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn url(&self) -> &str {
        self.url.as_ref().map_or(self.token.as_str(), |url| url)
    }

    pub fn full_url(&self) -> String {
        format!("{}/{}", self.host, self.url())
    }

    #[cfg(feature = "tls-server")]
    pub fn set_certificate_and_key(mut self, cert_and_key: CertAndKey, self_signed: bool) -> Self {
        self.cert_and_key = Some(cert_and_key);
        self.options.set(OptionFlags::SELF_SIGNED, self_signed);
        self
    }

    #[cfg(feature = "tls-server")]
    pub fn certificate_input_file(&self) -> Option<InputFile> {
        self.cert_and_key
            .as_ref()
            .map(|cert_and_key| cert_and_key.into())
    }

    pub fn set_send_set_webhook(mut self, send_set_webhook: bool) -> Self {
        self.options
            .set(OptionFlags::SEND_SET_WEBHOOK, send_set_webhook);
        self
    }
}

impl<F, H, UH> Actor for TelegramServer<F, H, UH>
where
    H: IntoUpdateHandler<Handler = UH> + 'static,
    UH: UpdateHandler + 'static,
    F: Fn() -> H + Send + Clone + 'static,
{
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramServer is alive");
        let token = self.token.clone();
        let url = self.url().to_owned();
        let apps = self.factory.clone();
        let telegram_api = TelegramApi::new(token, 10).start();
        let clone = telegram_api.clone();
        let mut server = HttpServer::new(move || {
            let apps = (apps)().into_handler();
            let telegram_api = clone.clone();
            let state = ReqState { telegram_api, apps };
            ActixApp::with_state(state).resource(&url, |r| r.method(Method::POST).with(handler))
        })
        .workers(self.threads)
        .server_hostname(self.host.clone());
        let mut set_webhook = SetWebhook::new(self.full_url());
        #[cfg(feature = "tls-server")]
        {
            match self.cert_and_key.as_ref() {
                Some(cert_and_key) => {
                    #[cfg(feature = "rust-tls")]
                    {
                        server = server
                            .bind_rustls(self.addr.clone(), cert_and_key.get_acceptor())
                            .unwrap();
                    }
                    #[cfg(feature = "ssl")]
                    {
                        server = server
                            .bind_ssl(self.addr.clone(), cert_and_key.get_acceptor())
                            .unwrap();
                    }
                    #[cfg(feature = "tls")]
                    {
                        server = server
                            .bind_tls(self.addr.clone(), cert_and_key.get_acceptor())
                            .unwrap();
                    }
                    if self.options.contains(OptionFlags::SELF_SIGNED) {
                        set_webhook.set_certificate(Some(cert_and_key));
                    }
                }
                _ => {
                    server = server.bind(self.addr.clone()).unwrap();
                }
            }
        }
        #[cfg(not(feature = "tls-server"))]
        {
            server = server.bind(self.addr.clone()).unwrap();
        }
        if self.options.contains(OptionFlags::SEND_SET_WEBHOOK) {
            telegram_api.do_send(set_webhook);
        }
        self.server = Some(server.start());
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramServer is stopped");
    }
}

impl<F, H, UH> Handler<SetWebhook> for TelegramServer<F, H, UH>
where
    H: IntoUpdateHandler<Handler = UH> + 'static,
    UH: UpdateHandler + 'static,
    F: Fn() -> H + Send + Clone + 'static,
{
    type Result = Box<Future<Item = True, Error = ()>>;

    fn handle(&mut self, msg: SetWebhook, _: &mut Context<Self>) -> Self::Result {
        let telegram_api = TelegramApi::new(self.token.clone(), 10).start();
        println!("set webhook {:?}", msg);
        Box::new(
            telegram_api
                .send(msg)
                .map_err(|err| debug!("err {:?}", err))
                .and_then(|response| response),
        )
    }
}

fn handler<H>((update, state): (Json<Update>, State<ReqState<H>>)) -> HttpResponseBuilder
where
    H: UpdateHandler + 'static,
{
    let msg = update.into_inner();
    debug!("TelegramServer.Update received {:?}", msg);
    match state.apps.handle(msg, &state.telegram_api) {
        Ok(()) => {
            debug!("handled");
        }
        Err(_) => {
            debug!("not handled");
        }
    }
    HttpResponse::Ok()
}
