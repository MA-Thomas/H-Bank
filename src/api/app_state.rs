use std::sync::{Arc, Mutex};
use crate::api::data_manager::DataManagerTrait;
use crate::api::execution_engine::ExecutionEngine;
use crate::api::archive_system::ArchiveSystem;
use crate::api::result_processor::ResultProcessor;

pub struct AppState {
    pub data_manager: Arc<dyn DataManagerTrait>,
    pub execution_engine: Arc<Mutex<ExecutionEngine>>,
    pub archive_system: Arc<ArchiveSystem>,
    pub result_processor: Arc<ResultProcessor>,
}