use std::error::Error;
use std::collections::HashMap;
use std::sync::RwLock;

use super::shared_models::CodeSubmission;

pub struct ArchiveSystem {
    archives: RwLock<HashMap<String, CodeSubmission>>,
}

impl ArchiveSystem {
    pub fn new() -> Self {
        ArchiveSystem {
            archives: RwLock::new(HashMap::new()),
        }
    }

    pub fn store_submission(&self, job_id: &str, submission: &CodeSubmission) -> Result<(), Box<dyn Error>> {
        let mut archives = self.archives.write().map_err(|e| format!("Failed to acquire write lock: {}", e))?;
        archives.insert(job_id.to_string(), submission.clone());
        println!("Archived code submission for job {}", job_id);
        Ok(())
    }

    pub fn retrieve_submission(&self, job_id: &str) -> Option<CodeSubmission> {
        let archives = self.archives.read().ok()?;
        archives.get(job_id).cloned()
    }
}