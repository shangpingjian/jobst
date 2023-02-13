use etcd_client::{Client, Error};
use crate::config::CONFIG;



pub async fn get_client()-> Result<Client, Error>{

    let mut client = Client::connect(&CONFIG.etcd.endpoints, None).await?;
    // put kv
    client.put("foo", "bar", None).await?;
    // get kv
    let resp = client.get("foo", None).await?;
    if let Some(kv) = resp.kvs().first() {
        println!("Get kv: {{{}: {}}}", kv.key_str()?, kv.value_str()?);
    }

    Ok(client)

}