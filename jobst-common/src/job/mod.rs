use std::str::FromStr;

use serde::{Serialize, Deserialize};
use serde_bytes::Bytes;
use serde_json::to_vec;

#[derive(Debug, Serialize, Deserialize)]
pub enum RunType {
    Sync,
    Async,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Valid,
    Invalid,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExecStatus {
    Pending,
    Done,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum JobType {
    HttpRequest,
    GrpcRequest,
    Command,
}


impl FromStr for JobType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HttpRequest" => Ok(JobType::HttpRequest),
            "GrpcRequest" => Ok(JobType::GrpcRequest),
            "Command" => Ok(JobType::Command),
            _ => Err(format!("Invalid variant: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub job_id: String,
    pub job_name: String,
    pub job_type: JobType,

}

impl Into<Vec<u8>> for Job{
    fn into(self) -> Vec<u8> {
        let bytes = to_vec(&self).unwrap();
        bytes
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct JobInstance {
    pub inst_id: String,
    pub job_info: Job,
    pub result: String,
    pub status: ExecStatus,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct JobQueue {
    pub queue_id: String,
    pub run_type: RunType,
    pub queue: Vec<Job>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobQueueInst {
    pub inst_id: String,
    pub queue_info: JobQueue,
    pub result: String,
    pub status: ExecStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobPile {
    pub pile_id: String,
    pub pile: Vec<JobQueue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobPileInst {
    pub inst_id: String,
    pub pile_info: JobPile,
    pub result: String,
    pub status: ExecStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobPlan {
    pub plan_id:String,
    pub plan_name: String,
    pub pile_id: String,
    pub expr: String,
}
