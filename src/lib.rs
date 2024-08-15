/*
Define the top level modules
Software Architecture Design Philosophy: Modular Monolith (perhaps migrating to Microservices as scale/problems demand in the future)
*/
pub mod contracts;
pub mod persons;
pub mod api;
pub mod execution;
pub mod endpoints; 

// Create an api_prelude module with only the necessary exports for the server app
/*
API users could do use h_bank::prelude::*; to get all common imports at once.
*/
pub mod api_prelude {
    pub use crate::api::shared_models::*;
    pub use crate::api::data_manager::DataManager;
    pub use crate::api::archive_system::ArchiveSystem;
    pub use crate::api::code_storage::ResultProcessor;
    pub use crate::api::app_state::*;
    pub use crate::endpoints::app_endpoints::*;
    pub use crate::execution::*;
}