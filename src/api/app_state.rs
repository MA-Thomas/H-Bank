use std::sync::Arc;
use crate::api::data_manager::DataManager;
use crate::api::code_storage::CodeStorage;
use crate::data_management::cohort_manager::CohortManager;
use crate::data_management::synthetic_data_generator::SyntheticDataGenerator;

pub struct AppState {
    pub data_manager: Arc<DataManager>,
    pub code_storage: Arc<CodeStorage>,
    pub cohort_manager: Arc<CohortManager>,
    pub synthetic_data_generator: Arc<SyntheticDataGenerator>,
}
