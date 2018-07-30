extern crate actix_web;
extern crate futures;

use futures::future::Future;
use actix_web::{actix, actix::System, client, HttpMessage};
use std::env;

fn main() {
    let token = env::var("TELEGRAM_TOKEN").unwrap();
    let method = "getMe";
    let url = format!("https://api.telegram.org/bot{}/{}", token, method);
    actix::run(||
        client::get(url)   // <- Create request builder
            .header("User-Agent", "Actix-web")
            .finish().unwrap()
            .send()                               // <- Send http request
            .map_err(|_| ())
            .and_then(|response| {                // <- server http response
                response
                    .body()
                    .limit(1024)
                    .map_err(|_| ())
                    .and_then(|body| {
                        println!("Response: {:?}", body);
                        System::current().stop();
                        Ok(())
                    })
            })
    );
}
