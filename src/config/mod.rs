use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub port: u32,
    pub read_time_out: u32,
    pub write_time_out: u32,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Etcd {
    pub endpoints: Vec<String>,
    pub dial_time_out: u32,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub etcd: Etcd,
}

lazy_static!(

    pub static ref CONFIG:Config = init_config();
);


pub fn init_config() -> Config {

    let file_res = File::open("config.yaml");
    let file = match file_res {
        Ok(f) => {f}
        Err(e) => {
            panic!("load config error: {}", e)
        }
    };
    let config:Config = match serde_yaml::from_reader(file){
        Ok(c) => {c},
        Err(e) => {
            panic!("load config error: {}", e)
        }
    };
    config
}
