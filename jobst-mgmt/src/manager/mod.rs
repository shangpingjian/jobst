mod app_config;
mod handlers;

use actix_web::{middleware, App, HttpServer};
use std::net::SocketAddrV4;
use jobst_common::config;

pub async fn start() -> std::io::Result<()> {

    let addr  = SocketAddrV4::new("127.0.0.1".parse().unwrap(), config::CONFIG.server.port);

    HttpServer::new(|| {
        App::new()
            .configure(app_config::config_app)
            .wrap(middleware::Logger::default())
    })
        .bind(addr)?
        .run()
        .await
}
