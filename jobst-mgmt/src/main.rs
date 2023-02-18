mod manager;

use jobst_common::config::log;

# [tokio::main]
async fn main() {


    log::logger_init();

    manager::start().await;

}
