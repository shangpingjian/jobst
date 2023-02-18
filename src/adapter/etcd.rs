use std::future::Future;
use crate::adapter::{Adapter, AdapterError};
use crate::storege::etcd;
use crate::job::{Job, JobInstance};
use etcd_client;
use async_trait::async_trait;
use etcd_client::{Client, Error};
use futures::executor::block_on;


const JOB_DIR: &str = "/JOB/";
const JOB_QUEUE_DIR: &str = "/JOB_QUEUE/";
const JOB_PILE_DIR: &str = "/JOB_PILE/";


pub struct EtcdAdapter {
    client: etcd_client::Client
}

impl EtcdAdapter{


    pub fn new()->EtcdAdapter{
        let client = block_on(etcd::get_client()).unwrap();

        EtcdAdapter{
            client
        }

    }

}


#[async_trait]
impl Adapter for EtcdAdapter {
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

    fn get_job_plan(&self) {
        println!("a")
    }

    fn get_job_pile(&self) {}

    async fn get_jobs(&self) -> Result<Vec<Job>, AdapterError> {
        let r: Result<etcd_client::GetResponse, etcd_client::Error> = etcd::get_all(&mut self.client.clone(), JOB_DIR.to_string()).await;
        match r {
            Ok(a) => {
                let mut jobs = vec![];
                for kv in a.kvs() {
                    let value = kv.value();
                    let b = std::str::from_utf8(value).unwrap();
                    let job_result = serde_json::from_str(b);
                    match job_result {
                        Ok(j) => { jobs.push(j) }
                        Err(e) => { continue; }
                    }
                }
                Ok(jobs)
            }
            Err(e) => Err(self.error_transfer(Box::new(e)))
        }
    }

    async fn create_job(&self, job: Job) -> Result<(), AdapterError> {
        let key = JOB_DIR.to_string() + &job.job_id;
        let value: Vec<u8> = job.into();
        let r: Result<(), etcd_client::Error> = etcd::put(&mut self.client.clone(), key, value).await;
        match r {
            Ok(_) => Ok(()),
            Err(e) => Err(self.error_transfer(Box::new(e)))
        }
    }

    async fn get_job(&self, job_id: String) -> Result<Job, AdapterError> {
        let key = JOB_DIR.to_string() + &job_id;
        let r: Result<etcd_client::GetResponse, etcd_client::Error> = etcd::get_all(&mut self.client.clone(), key).await;
        match r {
            Ok(a) => {
                let value = a.kvs()[0].value();
                let b = std::str::from_utf8(value).unwrap();
                let job_result = serde_json::from_str(b);
                match job_result {
                    Ok(j) => { return Ok(j); }
                    Err(e) => { Err(AdapterError::ParseError) }
                }
            }
            Err(e) => Err(self.error_transfer(Box::new(e)))
        }
    }

    async fn delete_job(&self, job_id: String) -> Result<(), AdapterError> {
        let key = JOB_DIR.to_string() + &job_id;
        let r: Result<etcd_client::DeleteResponse, etcd_client::Error> = etcd::delete(&mut self.client.clone(), key).await;
        match r {
            Ok(a) => Ok(()),
            Err(e) => Err(self.error_transfer(Box::new(e)))
        }
    }
}
