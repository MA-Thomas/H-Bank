use serde::{Serialize, Deserialize};
use std::path::PathBuf;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyntheticDataSetup {
    pub cohort_id: String,
    pub data_dir: PathBuf,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeSubmission {
    pub cohort_id: String,
    pub code: String,
    pub dependencies: Vec<String>,
    pub data_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalysisResult {
    pub job_id: String,
    pub status: String,
    pub result: Option<String>,
    pub error: Option<String>,
}

// Add any other shared structures here