pub mod api;
pub mod contracts;
pub mod persons;
pub mod data_management;

pub use api::HBankInterface;

// Create an api_prelude module with all necessary exports for the server app
pub mod api_prelude {
    pub use crate::api::shared_models::*;
    pub use crate::api::HBankInterface;
    pub use crate::contracts::DataPrivacyLevel;
    pub use crate::data_management::cohort_manager::CohortSummary;
}
