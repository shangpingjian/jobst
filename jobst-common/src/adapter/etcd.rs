use std::convert::Into;
use std::future::Future;
use crate::adapter::{AdapterError, JobAdapter};
use crate::storege::etcd as etcd_storge;
use crate::job::{Job};
use etcd_client;
use async_trait::async_trait;
use etcd_client::{Client, Error, GetResponse};
use futures::executor::block_on;
use serde::{Serialize, Deserialize};


const JOB_DIR: &str = "/JOB/";
const JOB_QUEUE_DIR: &str = "/JOB_QUEUE/";
const JOB_PILE_DIR: &str = "/JOB_PILE/";


pub struct EtcdAdapter {
    client: etcd_client::Client,
}

impl EtcdAdapter {
    pub fn new() -> EtcdAdapter {
        let client = block_on(etcd_storge::get_client()).unwrap();

        EtcdAdapter {
            client
        }
    }
}


impl EtcdAdapter {
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

    async fn get_list(&self) -> Result<Vec<String>, AdapterError> {
        let r: Result<etcd_client::GetResponse, etcd_client::Error> = etcd_storge::get_all(&mut self.client.clone(), JOB_DIR.to_string()).await;
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

    async fn create<'a, T: Serialize + Deserialize<'a> + Into<Vec<u8>> + From<Vec<u8>> + Send>(&'a self, key: String, job: T) -> Result<(), AdapterError> {
        let value: Vec<u8> = job.into();
        let r: Result<(), etcd_client::Error> = etcd_storge::put(&mut self.client.clone(), key, value).await;
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

    async fn delete(&self, key: String) -> Result<(), AdapterError> {
        let r: Result<etcd_client::DeleteResponse, etcd_client::Error> = etcd_storge::delete(&mut self.client.clone(), key).await;
        match r {
            Ok(a) => Ok(()),
            Err(e) => Err(self.error_transfer(Box::new(e)))
        }
    }
}


pub struct EtcdJobAdapter {
    adapter: EtcdAdapter,
}

impl EtcdJobAdapter {
    pub fn new() -> EtcdJobAdapter {
        let adapter = EtcdAdapter::new();
        EtcdJobAdapter {
            adapter,
        }
    }
}


#[async_trait]
impl JobAdapter for EtcdJobAdapter {
    async fn get_job_list(&self) -> Result<Vec<Job>, AdapterError> {
        let r = self.adapter.get_list().await;
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

    async fn create_job(&self, job: Job) -> Result<(), AdapterError> {
        self.adapter.create(job.job_id.clone(), job).await
    }

    async fn get_job_detail(&self, key: String) -> Result<Job, AdapterError> {
        let r = self.adapter.get_detail(key).await;
        match r {
            Ok(a) => {
                let job:Job = serde_json::from_str(&a).unwrap();
                Ok(job)
            },
            Err(e)=>Err(e)
        }
    }

    async fn delete_job(&self, key: String) -> Result<(), AdapterError> {
        let r = self.adapter.delete(key).await?;
        Ok(r)
    }
}

