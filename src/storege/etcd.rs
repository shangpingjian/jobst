use etcd_client::{Client, Error, GetResponse, PutResponse, DeleteResponse, Watcher, WatchStream, GetOptions};
use crate::config::CONFIG;
use lazy_static::lazy_static;
use futures::executor::block_on;


pub async fn get_client() -> Result<Client, Error>{

    let mut client = Client::connect(&CONFIG.etcd.endpoints, None).await?;
    Ok(client)

}

pub async fn put<K: Into<Vec<u8>>, V: Into<Vec<u8>>>(key: K, value: V) -> Result<(), Error>{
    // put kv
    let mut client = get_client().await?;
    let r = client.put(key, value, None).await?;
    Ok(())

    // get kv

}

pub async fn get<K: Into<Vec<u8>>>(key: K) -> Result<GetResponse, Error>{
    // put kv
    let mut client = get_client().await?;
    let r = client.get(key, None).await;
    r
    // get kv
}

pub async fn get_all<K: Into<Vec<u8>>>(key: K) -> Result<GetResponse, Error>{
    // put kv
    let mut client = get_client().await?;
    let option = GetOptions::new().with_prefix();
    let r = client.get(key, Option::from(option)).await;
    r
    // get kv
}

pub async fn delete<K: Into<Vec<u8>>>(key: K) -> Result<DeleteResponse, Error>{
    // put kv
    let mut client = get_client().await?;
    let r = client.delete(key, None).await;
    r
    // get kv
}

pub async fn watch<K: Into<Vec<u8>>>(key: K) -> Result<(Watcher, WatchStream), Error>{
    // put kv
    let mut client = get_client().await?;
    let r = client.watch(key, None).await;
    r
    // get kv
}
