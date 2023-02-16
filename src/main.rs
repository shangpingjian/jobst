mod config;
mod storege;
mod worker;
mod scheduler;
mod manager;
mod job;
mod listener;
mod adapter;


# [tokio::main]
async fn main() {


    config::log::logger_init();

    manager::start().await;

}
