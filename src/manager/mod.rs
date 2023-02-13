mod app_config;
mod handlers;

use actix_web::{middleware, App, HttpServer};

pub async fn start() -> std::io::Result<()> {


    HttpServer::new(|| {
        App::new()
            .configure(app_config::config_app)
            .wrap(middleware::Logger::default())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
