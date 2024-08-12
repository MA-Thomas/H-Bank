use std::error::Error;
use std::collections::HashMap;

use super::models::AnalysisResult;

pub struct ResultProcessor {
    results: HashMap<String, AnalysisResult>,
}

impl ResultProcessor {
    pub fn new() -> Self {
        ResultProcessor {
            results: HashMap::new(),
        }
    }

    pub fn process_result(&mut self, result: AnalysisResult) -> Result<(), Box<dyn Error>> {
        // Placeholder: In a real implementation, this would sanitize the result to ensure no sensitive data is leaked
        println!("Processing result for job {}", result.job_id);
        self.results.insert(result.job_id.clone(), result);
        Ok(())
    }

    pub fn get_processed_result(&self, job_id: &str) -> Result<String, Box<dyn Error>> {
        self.results.get(job_id)
            .map(|result| result.result.clone())
            .ok_or_else(|| "Result not found".into())
    }
}