use std::error::Error;
use std::collections::HashMap;

use super::models::AnalysisRequest;

pub struct ArchiveSystem {
    archives: HashMap<String, AnalysisRequest>,
}

impl ArchiveSystem {
    pub fn new() -> Self {
        ArchiveSystem {
            archives: HashMap::new(),
        }
    }

    pub fn store_submission(&mut self, job_id: &str, request: &AnalysisRequest) -> Result<(), Box<dyn Error>> {
        self.archives.insert(job_id.to_string(), request.clone());
        println!("Archived submission for job {}", job_id);
        Ok(())
    }

    pub fn retrieve_submission(&self, job_id: &str) -> Option<&AnalysisRequest> {
        self.archives.get(job_id)
    }
}