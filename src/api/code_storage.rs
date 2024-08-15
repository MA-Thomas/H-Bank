use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;
use crate::api::shared_models::CodeSubmission;

pub struct CodeStorage {
    submissions: RwLock<HashMap<String, CodeSubmission>>,
}

impl CodeStorage {
    pub fn new() -> Self {
        CodeStorage {
            submissions: RwLock::new(HashMap::new()),
        }
    }

    pub fn store_submission(&self, submission: CodeSubmission) -> String {
        let job_id = Uuid::new_v4().to_string();
        let mut submissions = self.submissions.write().unwrap();
        submissions.insert(job_id.clone(), submission);
        job_id
    }

    pub fn get_submission(&self, job_id: &str) -> Option<CodeSubmission> {
        let submissions = self.submissions.read().unwrap();
        submissions.get(job_id).cloned()
    }
}