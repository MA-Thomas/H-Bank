use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use super::{
    data_manager::DataManager,
    execution_engine::ExecutionEngine,
    archive_system::ArchiveSystem,
    result_processor::ResultProcessor,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalysisRequest {
    pub cohort_id: String,
    pub code_repository_url: String,
    pub environment_specs: EnvironmentSpecs,
    pub data_type: DataType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvironmentSpecs {
    pub language: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub resource_requirements: ResourceRequirements,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceRequirements {
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub disk_space_gb: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DataType {
    Synthetic,
    Sensitive,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AnalysisData {
    Synthetic(String),
    Sensitive(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatus {
    pub status: String,
    pub result_url: Option<String>,
}


pub struct AppState {
    pub data_manager: Arc<DataManager>,
    pub execution_engine: Arc<Mutex<ExecutionEngine>>,
    pub archive_system: Arc<ArchiveSystem>,
    pub result_processor: Arc<ResultProcessor>,
    pub jobs: Arc<Mutex<HashMap<String, JobStatus>>>,
}

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub job_id: String,
    pub result: String,
}