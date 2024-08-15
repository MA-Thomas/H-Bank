use std::sync::RwLock;
use std::collections::{HashMap, HashSet};
use crate::api::shared_models::AnalysisResult;

pub struct DataManager {
    analysis_results: RwLock<HashMap<String, AnalysisResult>>,
    valid_server_app_ids: RwLock<HashSet<String>>,
}

impl DataManager {
    pub fn new() -> Self {
        DataManager {
            analysis_results: RwLock::new(HashMap::new()),
            valid_server_app_ids: RwLock::new(HashSet::new()),
        }
    }

    pub fn store_analysis_result(&self, result: &AnalysisResult) -> Result<(), String> {
        let mut results = self.analysis_results.write().map_err(|e| e.to_string())?;
        results.insert(result.job_id.clone(), result.clone());
        Ok(())
    }

    pub fn get_analysis_result(&self, job_id: &str) -> Option<AnalysisResult> {
        let results = self.analysis_results.read().ok()?;
        results.get(job_id).cloned()
    }

    pub fn validate_server_app_id(&self, server_app_id: &str) -> bool {
        let valid_ids = self.valid_server_app_ids.read().unwrap();
        valid_ids.contains(server_app_id)
    }

    // Method to add valid server app IDs (you might want to make this more secure in a real-world scenario)
    pub fn add_valid_server_app_id(&self, server_app_id: String) {
        let mut valid_ids = self.valid_server_app_ids.write().unwrap();
        valid_ids.insert(server_app_id);
    }
}