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
    pub wasm_code: Vec<u8>,
    pub entry_point: String,
    pub data_dir: PathBuf,
    pub execution_mode: ExecutionMode,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalysisResult {
    pub job_id: String,
    pub status: String,
    pub result: Option<String>,
    pub error: Option<String>,
}

// Add any other shared structures here
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionMode {
    // Whether wasm module is executed on real data stored locally (on Borrower's server) or remote (on HBank server)
    Local,
    Remote,
}

