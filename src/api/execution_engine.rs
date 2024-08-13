use async_trait::async_trait;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::api::shared_models::{CodeSubmission, AnalysisResult};


/*
This script should set up the sandbox and run 
user submitted code on sensitive data

*/

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobStatus {
    pub status: String,
    pub result: Option<AnalysisResult>,
}

#[async_trait]
pub trait ExecutionEngineTrait: Send + Sync {
    async fn queue_job(&mut self, job_id: &str, submission: &CodeSubmission) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn get_job_status(&self, job_id: &str) -> Option<JobStatus>;
    async fn run_next_job(&mut self) -> Option<Result<AnalysisResult, Box<dyn std::error::Error + Send + Sync>>>;
}

pub struct ExecutionEngine {
    job_queue: Vec<(String, CodeSubmission)>,
    job_statuses: HashMap<String, JobStatus>,
}

impl ExecutionEngine {
    pub fn new() -> Self {
        ExecutionEngine {
            job_queue: Vec::new(),
            job_statuses: HashMap::new(),
        }
    }
}

#[async_trait]
impl ExecutionEngineTrait for ExecutionEngine {
    async fn queue_job(&mut self, job_id: &str, submission: &CodeSubmission) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.job_queue.push((job_id.to_string(), submission.clone()));
        self.job_statuses.insert(job_id.to_string(), JobStatus {
            status: "Queued".to_string(),
            result: None,
        });
        Ok(())
    }

    async fn get_job_status(&self, job_id: &str) -> Option<JobStatus> {
        self.job_statuses.get(job_id).cloned()
    }

    async fn run_next_job(&mut self) -> Option<Result<AnalysisResult, Box<dyn std::error::Error + Send + Sync>>> {
        // Implement job execution logic here
        None
    }
}