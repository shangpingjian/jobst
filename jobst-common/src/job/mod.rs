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
pub enum State {
    Valid,
    Invalid,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExecStatus {
    Pending,
    Processing,
    Done,
    Failed
}

impl FromStr for ExecStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(ExecStatus::Pending),
            "Processing" => Ok(ExecStatus::Processing),
            "Done" => Ok(ExecStatus::Done),
            "Failed" => Ok(ExecStatus::Failed),
            _ => Err(format!("Invalid variant: {}", s)),
        }
    }
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
    pub state: State,
    pub exec_status: ExecStatus

}

impl Into<Vec<u8>> for Job{
    fn into(self) -> Vec<u8> {
        let bytes = to_vec(&self).unwrap();
        bytes
    }
}

impl From<Vec<u8>> for Job {
    fn from(bytes: Vec<u8>) -> Self {
        let job: Job = serde_json::from_slice(&bytes).unwrap();
        job
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobQueue {
    pub queue_id: String,
    pub run_type: RunType,
    pub job_ids: Vec<String>,
    pub state: State,
    pub exec_status: ExecStatus

}

impl Into<Vec<u8>> for JobQueue{
    fn into(self) -> Vec<u8> {
        let bytes = to_vec(&self).unwrap();
        bytes
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobPile {
    pub pile_id: String,
    pub queue_ids: Vec<String>,
    pub run_type: RunType,
    pub state: State,
    pub exec_status: ExecStatus

}

impl Into<Vec<u8>> for JobPile{
    fn into(self) -> Vec<u8> {
        let bytes = to_vec(&self).unwrap();
        bytes
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobPlan {
    pub plan_id:String,
    pub plan_name: String,
    pub pile_id: String,
    pub cron_expr: String,
    pub state: State,
    pub exec_status: ExecStatus

}

impl Into<Vec<u8>> for JobPlan{
    fn into(self) -> Vec<u8> {
        let bytes = to_vec(&self).unwrap();
        bytes
    }
}
