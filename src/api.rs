mod interface;
mod data_manager;
mod archive_system;
mod code_storage;
pub mod shared_models;

pub use interface::HBankInterface;

// This will be used for future API implementation
pub mod api_endpoints {
    // This will be implemented in the future when adding API functionality
    pub fn configure() {
        // API route configuration will be added here later
    }
}
