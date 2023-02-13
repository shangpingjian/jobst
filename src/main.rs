mod config;
mod etcd;

# [tokio::main]
async fn main() {

    let ec = match etcd::get_client().await{
        Ok(c) => {c}
        Err(e) => {
            let err_msg = format!("get etcd connect error: {}", e);
            println!("{}", err_msg);
            panic!("{}", err_msg)
        }
    };



}
