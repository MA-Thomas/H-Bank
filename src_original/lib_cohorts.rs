use std::collections::HashMap;

use crate::lib_contracts::{HealthDataContract};
use crate::lib_contract_structs_enums::{ContractCategory, ContractLegalFramework, 
    DataCustodian, DataOriginator, DataRecipient, DonationLegalStructure, Donor, Funder, HBank,
    GeneratorRateSpecification, IndividualContributionLevel, 
    IsAgent, IsOriginator, IsRecipient, IsConsultant, IsDonor, IsFunder, IsGenerator, IsHBank, Party, 
    StorageLegalStructure, TransactionLegalStructure, TwoPartyLegalStructure};



/*
A cohort is a set of individuals (and their contracts) associated with a single borrower.
*/
pub struct CohortManager<A, B, C, D, E, F, G, O, H> {
    cohorts: HashMap<String, Vec<HealthDataContract<A, B, C, D, E, F, G, O, H>>>,  // Map from cohort_id to contracts
}

impl<A, B, C, D, E, F, G, O, H> CohortManager<A, B, C, D, E, F, G, O, H>
where 
    A: IsAgent + Party + 'static,
    B: IsAgent + IsRecipient + Party + 'static,
    C: IsConsultant + Party + 'static,
    D: IsDonor + Party + 'static,
    F: IsFunder + Party + 'static,
    G: IsGenerator + Party + 'static,
    O: IsAgent + IsOriginator + Party + 'static,
    H: IsHBank + Party + 'static,
{
    // Constructor
    pub fn new() -> Self {
        CohortManager {
            cohorts: HashMap::new(),
        }
    }

    // Method to add a contract to a cohort
    pub fn add_contract_to_cohort(&mut self, cohort_id: String, mut contract: HealthDataContract<A, B, C, D, E, F, G, O, H>) -> Result<(), String> {
        if let Some(contract_cohort_id) = &contract.cohort_id {
            if contract_cohort_id != &cohort_id {
                return Err(format!("Contract cohort_id {:?} does not match the provided cohort_id {}", contract_cohort_id, cohort_id));
            }
        } else {
            contract.cohort_id = Some(cohort_id.clone());
        }

        self.cohorts.entry(cohort_id).or_insert_with(Vec::new).push(contract);
        Ok(())
    }

    // Method to get contracts by cohort ID
    pub fn get_contracts_by_cohort(&self, cohort_id: &str) -> Option<&Vec<HealthDataContract<A, B, C, D, E, F, G, O, H>>> {
        self.cohorts.get(cohort_id)
    }

    // Method to remove a cohort
    pub fn remove_cohort(&mut self, cohort_id: &str) -> Option<Vec<HealthDataContract<A, B, C, D, E, F, G, O, H>>> {
        self.cohorts.remove(cohort_id)
    }

    // Method to set cohort_id of a contract to None
    pub fn set_cohort_id_to_none(&mut self, contract: &mut HealthDataContract<A, B, C, D, E, F, G, O, H>) {
        if let Some(cohort_id) = &contract.cohort_id {
            if let Some(contracts) = self.cohorts.get_mut(cohort_id) {
                if let Some(pos) = contracts.iter().position(|c| c == contract) {
                    contracts.remove(pos);
                }
            }
            contract.cohort_id = None;
        }
    }
}
