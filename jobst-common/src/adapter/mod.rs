use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::job::Job;
use std::future::Future;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

mod etcd;

pub enum AdapterType {
    Etcd
}

pub enum AdapterError {
    ArgsError,
    ConnectionError,
    ParseError,
    IOError,
    UnknownError,
}

impl Debug for AdapterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for AdapterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for AdapterError {}


#[async_trait]
pub trait JobAdapter {
    async fn get_job_list(&self) -> Result<Vec<Job>, AdapterError>;
    async fn create_job(&self, job: Job) -> Result<(), AdapterError>;
    async fn get_job_detail(&self, key: String) -> Result<Job, AdapterError>;
    async fn delete_job(&self, key: String) -> Result<(), AdapterError>;
}

pub fn new_job_adapter(adater_type: AdapterType) -> Box<dyn JobAdapter> {
    match adater_type {
        AdapterType::Etcd => Box::new(etcd::EtcdJobAdapter::new()),
    }
}
