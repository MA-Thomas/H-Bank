use super::cohort_manager::CohortManager;
use crate::contracts::health_data_contract::HealthDataContract;
use crate::contracts::structs_enums::{DataPrivacyLevel, ContractLegalFramework};
use std::collections::HashMap;

// We'll use a type alias to simplify our function signatures
type GenericContract = HealthDataContract<Box<dyn IsAgent>, Box<dyn IsRecipient>, Box<dyn IsConsultant>, Box<dyn IsDonor>, Box<dyn IsAdvertiser>, Box<dyn IsFunder>, Box<dyn IsGenerator>, Box<dyn IsOriginator>, Box<dyn IsHBank>>;

pub struct CohortSummary {
    pub contract_count: usize,
    pub privacy_levels: HashMap<DataPrivacyLevel, usize>,
    pub legal_frameworks: HashMap<ContractLegalFramework, usize>,
}

// Global CohortManager instance
lazy_static! {
    static ref COHORT_MANAGER: Mutex<CohortManager<Box<dyn IsAgent>, Box<dyn IsRecipient>, Box<dyn IsConsultant>, Box<dyn IsDonor>, Box<dyn IsAdvertiser>, Box<dyn IsFunder>, Box<dyn IsGenerator>, Box<dyn IsOriginator>, Box<dyn IsHBank>>> = Mutex::new(CohortManager::new());
}

pub fn add_contract_to_cohort(cohort_id: String, contract: GenericContract) -> Result<(), String> {
    let mut manager = COHORT_MANAGER.lock().unwrap();
    manager.add_contract_to_cohort(cohort_id, contract)
}

pub fn get_contract_info(cohort_id: &str) -> Option<Vec<(String, DataPrivacyLevel)>> {
    let manager = COHORT_MANAGER.lock().unwrap();
    manager.get_contracts_by_cohort(cohort_id).map(|contracts| {
        contracts.iter().map(|contract| {
            (contract.contract_id.clone(), contract.privacy_level.clone())
        }).collect()
    })
}

pub fn get_cohort_summary(cohort_id: &str) -> Option<CohortSummary> {
    let manager = COHORT_MANAGER.lock().unwrap();
    manager.get_contracts_by_cohort(cohort_id).map(|contracts| {
        let mut summary = CohortSummary {
            contract_count: contracts.len(),
            privacy_levels: HashMap::new(),
            legal_frameworks: HashMap::new(),
        };

        for contract in contracts {
            *summary.privacy_levels.entry(contract.privacy_level.clone()).or_insert(0) += 1;
            *summary.legal_frameworks.entry(contract.legal_framework.clone()).or_insert(0) += 1;
        }

        summary
    })
}

pub fn remove_cohort(cohort_id: &str) -> Option<Vec<GenericContract>> {
    let mut manager = COHORT_MANAGER.lock().unwrap();
    manager.remove_cohort(cohort_id)
}