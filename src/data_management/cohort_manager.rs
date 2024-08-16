use std::collections::{HashMap,HashSet};
use crate::contracts::health_data_contract::HealthDataContract;
use crate::contracts::structs_enums::{EntityId, DataPrivacyLevel, Party};
use serde::{Serialize, Deserialize};

pub struct CohortManager {
    cohorts: HashMap<String, Cohort>,
}

pub struct Cohort {
    cohort_id: String,
    contracts: Vec<HealthDataContract>,
    privacy_level: DataPrivacyLevel,
    total_participants: usize,
}

impl CohortManager {
    pub fn new() -> Self {
        CohortManager {
            cohorts: HashMap::new(),
        }
    }

    pub fn create_cohort(&mut self, cohort_id: String, privacy_level: DataPrivacyLevel) -> Result<(), String> {
        if self.cohorts.contains_key(&cohort_id) {
            return Err(format!("Cohort with ID {} already exists", cohort_id));
        }

        let new_cohort = Cohort {
            cohort_id: cohort_id.clone(),
            contracts: Vec::new(),
            privacy_level,
            total_participants: 0,
        };

        self.cohorts.insert(cohort_id, new_cohort);
        Ok(())
    }

    pub fn add_contract_to_cohort(&mut self, cohort_id: &str, contract: HealthDataContract) -> Result<(), String> {
        let cohort = self.cohorts.get_mut(cohort_id).ok_or_else(|| format!("Cohort with ID {} not found", cohort_id))?;

        if cohort.privacy_level != *contract.get_privacy_level() {
            return Err("Contract privacy level does not match cohort privacy level".to_string());
        }

        cohort.contracts.push(contract);
        cohort.update_total_participants();
        Ok(())
    }

    pub fn remove_contract_from_cohort(&mut self, cohort_id: &str, contract_id: &str) -> Result<(), String> {
        let cohort = self.cohorts.get_mut(cohort_id).ok_or_else(|| format!("Cohort with ID {} not found", cohort_id))?;

        let contract_index = cohort.contracts.iter().position(|c| c.get_contract_id() == contract_id)
            .ok_or_else(|| format!("Contract with ID {} not found in cohort {}", contract_id, cohort_id))?;

        cohort.contracts.remove(contract_index);
        cohort.update_total_participants();
        Ok(())
    }

    pub fn get_cohort_summary(&self, cohort_id: &str) -> Result<CohortSummary, String> {
        let cohort = self.cohorts.get(cohort_id).ok_or_else(|| format!("Cohort with ID {} not found", cohort_id))?;

        Ok(CohortSummary {
            cohort_id: cohort.cohort_id.clone(),
            privacy_level: cohort.privacy_level.clone(),
            total_participants: cohort.total_participants,
            contract_count: cohort.contracts.len(),
        })
    }

    pub fn list_cohorts(&self) -> Vec<String> {
        self.cohorts.keys().cloned().collect()
    }
}

impl Cohort {
    fn update_total_participants(&mut self) {
        self.total_participants = self.contracts.iter()
            .flat_map(|contract| contract.get_parties().iter())
            .filter(|party| matches!(party, Party::DataOriginator(_)))
            .map(|party| match party {
                Party::DataOriginator(info) => &info.entity_id,
                _ => unreachable!(),
            })
            .collect::<HashSet<&EntityId>>()
            .len();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortSummary {
    pub cohort_id: String,
    pub privacy_level: DataPrivacyLevel,
    pub total_participants: usize,
    pub contract_count: usize,
}