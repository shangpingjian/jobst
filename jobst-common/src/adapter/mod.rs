use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;

use actix_web::web::Query;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use crate::config::EngineType;
use crate::job::{Job, JobPile, JobPlan, JobQueue};

mod etcd;

pub enum AdapterType {
    Etcd
}

impl From<EngineType> for AdapterType{
    fn from(value: EngineType) -> Self {
        match value {
            EngineType::Etcd => AdapterType::Etcd,
        }
    }
}

pub enum AdapterDataType {
    Job,
    Queue,
    Pile,
    Plan,
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
pub trait JobAdapterTrait {
    async fn delete(&self, key: String, data_type:AdapterDataType) -> Result<(), AdapterError>;

    async fn get_job_list(&self) -> Result<Vec<Job>, AdapterError>;
    async fn get_job_detail(&self, key: String) -> Result<Job, AdapterError>;
    async fn create_job(&self, key:String, job:Job) -> Result<(), AdapterError>;

    async fn get_queue_list(&self) -> Result<Vec<JobQueue>, AdapterError>;
    async fn get_queue_detail(&self, key: String) -> Result<JobQueue, AdapterError>;
    async fn create_queue(&self, key:String, queue:JobQueue) -> Result<(), AdapterError>;

    async fn get_pile_list(&self) -> Result<Vec<JobPile>, AdapterError>;
    async fn get_pile_detail(&self, key: String) -> Result<JobPile, AdapterError>;
    async fn create_pile(&self, key:String, pile:JobPile) -> Result<(), AdapterError>;

    async fn get_plan_list(&self) -> Result<Vec<JobPlan>, AdapterError>;
    async fn get_plan_detail(&self, key: String) -> Result<JobPlan, AdapterError>;
    async fn create_plan(&self, key:String, plan:JobPlan) -> Result<(), AdapterError>;

}

pub fn new_job_adapter(adater_type: AdapterType) -> Box<dyn JobAdapterTrait> {
    match adater_type {
        AdapterType::Etcd => Box::new(etcd::EtcdAdapter::new()),
    }
}
