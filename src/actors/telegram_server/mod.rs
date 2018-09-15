#[cfg(feature = "tls-server")]
mod tls_server;
mod types;

use self::types::ReqState;
use super::{App, TelegramApi};
use actix::{Actor, Addr, Context, Handler};
use actix_web::{
    http::Method,
    server::{HttpServer, Server},
    App as ActixApp, HttpResponse, Json, State,
};
use futures::Future;
use methods::SetWebhook;
use std::sync::Arc;
use types::{True, Update};

pub use self::types::ServerSetWebhook;

#[cfg(feature = "tls-server")]
pub use self::tls_server::*;

pub struct TelegramServer {
    addr: String,
    host: Option<String>,
    url: Option<String>,
    token: String,
    threads: usize,
    apps: Arc<Vec<App>>,
    server: Option<Addr<Server>>,
    #[cfg(feature = "tls-server")]
    certificate: Option<CertAndKey>,
}

impl TelegramServer {
    pub fn new(addr: String, token: String, apps: Vec<App>) -> Self {
        TelegramServer {
            addr,
            host: None,
            url: None,
            threads: 1,
            apps: Arc::new(apps),
            server: None,
            token,
            #[cfg(feature = "tls-server")]
            certificate: None,
        }
    }

    pub fn workers(mut self, num: usize) -> Self {
        self.threads = num;
        self
    }

    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }

    pub fn set_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    fn url(&self) -> &str {
        self.url.as_ref().map_or(&self.token, |url| url)
    }

    #[cfg(feature = "tls-server")]
    pub fn certificate_and_key(mut self, certificate: CertAndKey) -> Self {
        self.certificate = Some(certificate);
        self
    }
}

impl Actor for TelegramServer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramServer is alive");
        let token = self.token.clone();
        let url = self.url().to_owned();
        let apps = self.apps.clone();
        let mut server = HttpServer::new(move || {
            let telegram_api = TelegramApi::new((&token).to_owned(), 10).start();
            let apps = apps.clone();
            let state = ReqState { telegram_api, apps };
            ActixApp::with_state(state).resource(&url, |r| {
                r.method(Method::POST)
                    .with(|(update, state): (Json<Update>, State<ReqState>)| {
                        let mut msg = update.into_inner();
                        debug!("TelegramServer.Update received {:?}", msg);
                        for app in state.apps.iter() {
                            msg = match (app.0)(msg, &state.telegram_api) {
                                Ok(()) => {
                                    debug!("ok");
                                    return HttpResponse::Ok();
                                }
                                Err(msg) => {
                                    debug!("next");
                                    msg
                                }
                            };
                        }
                        HttpResponse::Ok()
                    })
            })
        }).workers(self.threads);
        if let Some(host) = self.host.clone() {
            server = server.server_hostname(host);
        }
        #[cfg(feature = "tls-server")]
        {
            match self.certificate.as_ref() {
                Some(certificate) => {
                    server = server
                        .bind_with(self.addr.clone(), certificate.get_acceptor())
                        .unwrap();
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
        self.server = Some(server.start());
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        debug!("TelegramServer is stopped");
    }
}

impl Handler<ServerSetWebhook> for TelegramServer {
    type Result = Box<Future<Item = True, Error = ()>>;

    fn handle(&mut self, msg: ServerSetWebhook, _: &mut Context<Self>) -> Self::Result {
        let telegram_api = TelegramApi::new(self.token.clone(), 10).start();
        let url = self
            .host
            .as_ref()
            .map_or(String::new(), |host| format!("{}/{}", host, self.url()));
        #[cfg(feature = "tls-server")]
        let certificate = if msg.send_certificate {
            self
                .certificate
                .as_ref()
                .map(|certificate| certificate.cert.input_file())
        } else {
            None
        };
        #[cfg(not(feature = "tls-server"))]
        let certificate = None;
        let set_webhook = SetWebhook {
            url,
            certificate,
            max_connections: msg.max_connections,
            allowed_updates: msg.allowed_updates,
        };
        println!("set webhook {:?}", set_webhook);
        Box::new(
            telegram_api
                .send(set_webhook)
                .map_err(|err| debug!("err {:?}", err))
                .and_then(|response| response),
        )
    }
}
