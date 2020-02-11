#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

pub mod config;
pub mod errors;
pub mod schema;
pub mod models;
pub mod handlers;

use actix_web::{App, HttpServer};
use listenfd::ListenFd;
use std::env;
use config::database;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init();
    database::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .configure(handlers::user_handler::init_routes)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        },
    };
    
    info!("Starting server");
    server.run().await
}
