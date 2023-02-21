use std::convert::Into;
use std::future::Future;
use crate::adapter::{AdapterError, JobAdapterTrait, AdapterDataType};
use crate::storege::etcd as etcd_storge;
use crate::job::{Job, JobPile, JobPlan, JobQueue};
use etcd_client;
use async_trait::async_trait;
use etcd_client::{Client, Error, GetResponse};
use etcd_client::SortTarget::Key;
use futures::executor::block_on;
use serde::{Serialize, Deserialize};


const JOB_DIR: &str = "/JOB/";
const JOB_QUEUE_DIR: &str = "/JOB_QUEUE/";
const JOB_PILE_DIR: &str = "/JOB_PILE/";
const JOB_PLAN_DIR: &str = "/JOB_PLAN/";


pub struct EtcdAdapter {
    client: Client,
}

impl EtcdAdapter {
    pub fn new() -> EtcdAdapter {
        let client = block_on(etcd_storge::get_client()).unwrap();
        EtcdAdapter {
            client,
        }
    }

    fn error_transfer(&self, source: Box<dyn std::error::Error>) -> AdapterError {
        match source.downcast::<etcd_client::Error>() {
            Ok(etcd_err) => match *etcd_err {
                etcd_client::Error::InvalidArgs(_) | etcd_client::Error::InvalidUri(_) | etcd_client::Error::InvalidHeaderValue(_) => AdapterError::ArgsError,
                etcd_client::Error::IoError(_) | etcd_client::Error::TransportError(_) | etcd_client::Error::GRpcStatus(_) | etcd_client::Error::LeaseKeepAliveError(_) => AdapterError::ConnectionError,
                etcd_client::Error::Utf8Error(_) => AdapterError::ParseError,
                etcd_client::Error::EndpointError(_) | etcd_client::Error::WatchError(_) | etcd_client::Error::ElectError(_) => AdapterError::UnknownError,
            },
            Err(_) => AdapterError::UnknownError
        }
    }

    async fn get_list(&self, key: String) -> Result<Vec<String>, AdapterError> {
        let r: Result<etcd_client::GetResponse, etcd_client::Error> = etcd_storge::get_all(&mut self.client.clone(), key).await;
        match r {
            Ok(a) => {
                let mut jobs = vec![];
                for kv in a.kvs() {
                    let value = kv.value();
                    let b = std::str::from_utf8(value).unwrap();
                    jobs.push(b.to_string())
                }
                Ok(jobs)
            }
            Err(e) => Err(self.error_transfer(Box::new(e)))
        }
    }

    async fn create<'a, T: Serialize + Deserialize<'a> + Into<Vec<u8>> + From<Vec<u8>> + Send>(&'a self, key: String, value: T, data_type: AdapterDataType) -> Result<(), AdapterError> {
        let full_key = match data_type {
            AdapterDataType::Job => { String::from(JOB_DIR) + key.as_str() }
            AdapterDataType::Queue => { String::from(JOB_QUEUE_DIR) + key.as_str() }
            AdapterDataType::Pile => { String::from(JOB_PILE_DIR) + key.as_str() }
            AdapterDataType::Plan => { String::from(JOB_PLAN_DIR) + key.as_str() }
        };
        let value: Vec<u8> = value.into();
        let r: Result<(), etcd_client::Error> = etcd_storge::put(&mut self.client.clone(), full_key, value).await;
        match r {
            Ok(_) => Ok(()),
            Err(e) => Err(self.error_transfer(Box::new(e)))
        }
    }

    async fn get_detail(&self, key: String) -> Result<String, AdapterError> {
        let r: Result<etcd_client::GetResponse, etcd_client::Error> = etcd_storge::get(&mut self.client.clone(), key).await;
        match r {
            Ok(a) => {
                let value = a.kvs()[0].value();
                let b = std::str::from_utf8(value).unwrap();
                Ok(b.to_string())
            }
            Err(e) => Err(self.error_transfer(Box::new(e)))
        }
    }
}


#[async_trait]
impl JobAdapterTrait for EtcdAdapter {
    async fn delete(&self, key: String, data_type: AdapterDataType) -> Result<(), AdapterError> {
        let full_key = match data_type {
            AdapterDataType::Job => { String::from(JOB_DIR) + key.as_str() }
            AdapterDataType::Queue => { String::from(JOB_QUEUE_DIR) + key.as_str() }
            AdapterDataType::Pile => { String::from(JOB_PILE_DIR) + key.as_str() }
            AdapterDataType::Plan => { String::from(JOB_PLAN_DIR) + key.as_str() }
        };
        let r: Result<etcd_client::DeleteResponse, etcd_client::Error> = etcd_storge::delete(&mut self.client.clone(), full_key).await;
        match r {
            Ok(a) => Ok(()),
            Err(e) => Err(self.error_transfer(Box::new(e)))
        }
    }

    async fn get_job_list(&self) -> Result<Vec<Job>, AdapterError> {
        let r = self.get_list(JOB_DIR.to_string()).await;
        match r {
            Ok(a) => {
                let mut jobs = vec![];
                for s in a {
                    let job_r = serde_json::from_str(&s);
                    match job_r {
                        Ok(j) => { jobs.push(j) }
                        Err(_) => { continue; }
                    }
                }
                Ok(jobs)
            }
            Err(e) => Err(e)
        }
    }

    async fn get_job_detail(&self, key: String) -> Result<Job, AdapterError> {
        let key = JOB_DIR.to_string() + &key;
        let r = self.get_detail(key).await;
        match r {
            Ok(a) => {
                let job: Job = serde_json::from_str(&a).unwrap();
                Ok(job)
            }
            Err(e) => Err(e)
        }
    }

    async fn create_job(&self, key: String, job: Job) -> Result<(), AdapterError> {
        self.create(key, job, AdapterDataType::Job).await
    }

    async fn get_queue_list(&self) -> Result<Vec<JobQueue>, AdapterError> {
        let r = self.get_list(JOB_QUEUE_DIR.to_string()).await;
        match r {
            Ok(a) => {
                let mut queues = vec![];
                for s in a {
                    let queue_r = serde_json::from_str(&s);
                    match queue_r {
                        Ok(q) => { queues.push(q) }
                        Err(_) => { continue; }
                    }
                }
                Ok(queues)
            }
            Err(e) => Err(e)
        }
    }

    async fn get_queue_detail(&self, key: String) -> Result<JobQueue, AdapterError> {
        let key = JOB_QUEUE_DIR.to_string() + &key;
        let r = self.get_detail(key).await;
        match r {
            Ok(a) => {
                let q: JobQueue = serde_json::from_str(&a).unwrap();
                Ok(q)
            }
            Err(e) => Err(e)
        }
    }

    async fn create_queue(&self, key: String, queue: JobQueue) -> Result<(), AdapterError> {
        self.create(key, queue, AdapterDataType::Queue).await
    }

    async fn get_pile_list(&self) -> Result<Vec<JobPile>, AdapterError> {
        let r = self.get_list(JOB_QUEUE_DIR.to_string()).await;
        match r {
            Ok(a) => {
                let mut piles = vec![];
                for s in a {
                    let pile_r = serde_json::from_str(&s);
                    match pile_r {
                        Ok(p) => { piles.push(p) }
                        Err(_) => { continue; }
                    }
                }
                Ok(piles)
            }
            Err(e) => Err(e)
        }
    }

    async fn get_pile_detail(&self, key: String) -> Result<JobPile, AdapterError> {
        let key = JOB_PILE_DIR.to_string() + &key;
        let r = self.get_detail(key).await;
        match r {
            Ok(a) => {
                let p: JobPile = serde_json::from_str(&a).unwrap();
                Ok(p)
            }
            Err(e) => Err(e)
        }
    }

    async fn create_pile(&self, key: String, pile: JobPile) -> Result<(), AdapterError> {
        self.create(key, pile, AdapterDataType::Pile).await
    }

    async fn get_plan_list(&self) -> Result<Vec<JobPlan>, AdapterError> {
        let r = self.get_list(JOB_PLAN_DIR.to_string()).await;
        match r {
            Ok(a) => {
                let mut plans = vec![];
                for s in a {
                    let pile_r = serde_json::from_str(&s);
                    match pile_r {
                        Ok(p) => { plans.push(p) }
                        Err(_) => { continue; }
                    }
                }
                Ok(plans)
            }
            Err(e) => Err(e)
        }
    }


    async fn get_plan_detail(&self, key: String) -> Result<JobPlan, AdapterError> {
        let key = JOB_PLAN_DIR.to_string() + &key;
        let r = self.get_detail(key).await;
        match r {
            Ok(a) => {
                let p: JobPlan = serde_json::from_str(&a).unwrap();
                Ok(p)
            }
            Err(e) => Err(e)
        }
    }

    async fn create_plan(&self, key: String, plan: JobPlan) -> Result<(), AdapterError> {
        self.create(key, plan, AdapterDataType::Plan).await
    }
}
