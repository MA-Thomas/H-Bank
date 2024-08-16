pub mod contracts;
pub mod persons;
pub mod api;
pub mod data_management;

// Create an api_prelude module with all necessary exports for the server app
pub mod api_prelude {
    pub use crate::api::shared_models::*;
    pub use crate::api::data_manager::DataManager;
    pub use crate::api::archive_system::ArchiveSystem;
    pub use crate::api::code_storage::*;
    pub use crate::api::app_state::*;
    pub use crate::api::api_endpoints::*;
    pub use crate::contracts::DataPrivacyLevel;
    
    // Add this line to include CohortSummary in the prelude
    pub use crate::data_management::cohort_manager::CohortSummary;
}
