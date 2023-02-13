use etcd_client::{Client, Error};
use crate::config::CONFIG;


pub async fn get_client()-> Result<Client, Error>{

    let mut client = Client::connect(&CONFIG.etcd.endpoints, None).await?;
    Ok(client)

}