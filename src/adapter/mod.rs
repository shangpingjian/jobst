use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::job::Job;
use std::future::Future;
use async_trait::async_trait;

mod etcd;

pub enum AdapterType{

    Etcd
}

pub enum AdapterError{
    ArgsError,
    ConnectionError,
    EncodeError,
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

impl Error for AdapterError{

}

#[async_trait]
pub trait Adapter {
    fn error_transfer(&self, source: Box<dyn Error>)->AdapterError;
    fn get_job_plan(&self);
    fn get_job_pile(&self);
    async fn get_jobs(&self)->Result<Vec<Job>, AdapterError>;
    async fn create_job(&self, job: Job)->Result<(), AdapterError>;


}


pub fn new(adater_type: AdapterType)-> Box<dyn Adapter> {
    match adater_type {
        AdapterType::Etcd=>Box::new(etcd::EtcdAdapter{}),
    }
}