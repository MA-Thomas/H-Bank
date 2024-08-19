use std::path::PathBuf;
use super::data_manager::DataManager;
use super::archive_system::ArchiveSystem;
use super::code_storage::CodeStorage;
use crate::data_management::cohort_manager::CohortManager;
use crate::data_management::synthetic_data_generator::SyntheticDataGenerator;
use super::shared_models::*;
use crate::api_prelude::CohortSummary;

pub struct HBankInterface {
    data_manager: DataManager,
    archive_system: ArchiveSystem,
    code_storage: CodeStorage,
    cohort_manager: CohortManager,
    synthetic_data_generator: SyntheticDataGenerator,
}

impl HBankInterface {
    pub fn new(base_data_path: PathBuf) -> Self {
        Self {
            data_manager: DataManager::new(),
            archive_system: ArchiveSystem::new(),
            code_storage: CodeStorage::new(),
            cohort_manager: CohortManager::new(),
            synthetic_data_generator: SyntheticDataGenerator::new(base_data_path),
        }
    }

    pub fn submit_code(&self, submission: CodeSubmission) -> Result<String, String> {
        Ok(self.code_storage.store_submission(submission))
    }

    pub fn get_code_submission(&self, job_id: &str) -> Result<CodeSubmission, String> {
        self.code_storage.get_submission(job_id)
            .ok_or_else(|| "Code submission not found".to_string())
    }

    pub fn get_cohort_summary(&self, cohort_id: &str) -> Result<CohortSummary, String> {
        self.cohort_manager.get_cohort_summary(cohort_id)
    }

    pub fn setup_synthetic_data(&self, setup: SyntheticDataSetup) -> Result<SyntheticDataSetup, String> {
        self.synthetic_data_generator.setup_synthetic_data(&setup)
            .map_err(|e| e.to_string())
    }

    pub fn submit_analysis_result(&self, result: AnalysisResult) -> Result<(), String> {
        self.data_manager.store_analysis_result(&result)
    }

    // Add other methods as needed...
}