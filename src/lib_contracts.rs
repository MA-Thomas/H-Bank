use std::any::{Any, TypeId};

use std::marker::PhantomData;

use crate::lib_contract_structs_enums::{IsHBank, IsAgent, IsConsultant, IsGenerator, IsFunder, IsDonor, Party,
    HBank, DataOriginator, DataCustodian, DataRecipient, Funder, Donor,
    StorageLegalStructure, DonationLegalStructure, TransactionLegalStructure,
    TwoPartyLegalStructure, ContractCategory,
    ContractLegalFramework};


// Function to check if a party matches a type (useful for checking if party to be added is compatible with agreement_type)
// fn party_is<T: Party>(party: &Box<dyn Party>) -> bool {
//     TypeId::of::<T>() == party.type_id()
// }
fn party_is<T: Party>(party: &dyn Party) -> bool {
    TypeId::of::<T>() == party.type_id()
}

// ********* BRING IT ALL TOGETHER. USE THE DEFINED DATA TYPES TO DEFINE A HEALTHDATACONTRACT ********* 
pub struct HealthDataContract<A, B, C, D, F, G, H> {
    parties: Vec<Box<dyn Party>>,
    agreement_type: ContractCategory,
    legal_framework: ContractLegalFramework,
    terms: String,
    _phantom: PhantomData<(A, B, C, D, F, G, H)>, // PhantomData to indicate unused type parameters (they will be used later for type checking)
}


impl<A, B, C, D, F, G, H> HealthDataContract<A, B, C, D, F, G, H> 
where 
    A: IsAgent + Party + 'static,
    B: IsAgent + Party + 'static,
    C: IsConsultant + Party + 'static,
    D: IsDonor + Party + 'static,
    F: IsFunder + Party + 'static,
    G: IsGenerator + Party + 'static,
    H: IsHBank + Party + 'static,
{
    pub fn new(
        parties: Vec<Box<dyn Party>>,
        agreement_type: ContractCategory,
        legal_framework: ContractLegalFramework,
        terms: String,
    ) -> Self {
        HealthDataContract {
            parties,
            agreement_type,
            legal_framework,
            terms,
            _phantom: PhantomData, // Initialize PhantomData without any value
        }
    }

    pub fn add_terms(&mut self, terms: &str) {
        self.terms.push_str(terms);
    }

    pub fn add_parties(&mut self, parties: Vec<Box<dyn Party>>) {
        match &self.agreement_type {
            ContractCategory::TwoParty(two_party_type) => {
                match two_party_type {
                    TwoPartyLegalStructure::Storage(storage_type) => {
                        if parties.len() != 2 {
                            panic!("Storage agreement requires exactly 2 parties.");
                        }
    
                        let agent_found = parties.iter().any(|party| party_is::<A>(&**party));
                        let generators_found = parties.iter().any(|party| party_is::<G>(&**party));
                        let h_bank_found = parties.iter().any(|party| party_is::<H>(&**party));

                        if (!agent_found && !generators_found) || !h_bank_found {
                            panic!("Parties do not match required types for StorageAgreement");
                        }
                    },
                    TwoPartyLegalStructure::Donation(donation_type) => {
                        if parties.len() != 2 {
                            panic!("Donation agreement requires exactly 2 parties.");
                        }
    
                        let donor_found = parties.iter().any(|party| party_is::<D>(&**party));
                        let h_bank_found = parties.iter().any(|party| party_is::<H>(&**party));
    
                        if !donor_found || !h_bank_found {
                            panic!("Parties do not match required types for Donation");
                        }
                    },
                }
            },
            ContractCategory::ThreePlusParty(three_party_type) => {
                match three_party_type {
                    TransactionLegalStructure::ConsultAgreement { .. } => {
                        if parties.len() < 4 {
                            panic!("Consult agreement requires at least 4 parties.");
                        }
    
                        let agent_found = parties.iter().any(|party| party_is::<A>(&**party));
                        let consultant_found = parties.iter().any(|party| party_is::<C>(&**party));
                        let generators_found = parties.iter().any(|party| party_is::<G>(&**party));
                        let h_bank_found = parties.iter().any(|party| party_is::<H>(&**party));
    
                        if !agent_found || !consultant_found || !generators_found || !h_bank_found {
                            panic!("Parties do not match required types for ConsultAgreement");
                        }
                    },
                    TransactionLegalStructure::DirectSale { .. } => {
                        if parties.len() < 4 {
                            panic!("Direct sale agreement requires at least 4 parties.");
                        }
    
                        let agent_a_found = parties.iter().any(|party| party_is::<A>(&**party));
                        let agent_b_found = parties.iter().any(|party| party_is::<B>(&**party));
                        let generators_found = parties.iter().any(|party| party_is::<G>(&**party));
                        let h_bank_found = parties.iter().any(|party| party_is::<H>(&**party));
    
                        if !agent_a_found || !agent_b_found || !generators_found || !h_bank_found {
                            panic!("Parties do not match required types for DirectSale");
                        }
                    },
                    TransactionLegalStructure::PurchaseAgreement { .. } => {
                        if parties.len() < 4 {
                            panic!("Purchase agreement requires at least 4 parties.");
                        }
    
                        let agent_a_found = parties.iter().any(|party| party_is::<A>(&**party));
                        let agent_b_found = parties.iter().any(|party| party_is::<B>(&**party));
                        let generators_found = parties.iter().any(|party| party_is::<G>(&**party));
                        let h_bank_found = parties.iter().any(|party| party_is::<H>(&**party));
    
                        if !agent_a_found || !agent_b_found || !generators_found || !h_bank_found {
                            panic!("Parties do not match required types for PurchaseAgreement");
                        }
                    },
                    TransactionLegalStructure::LicensingAgreement { .. } => {
                        if parties.len() < 4 {
                            panic!("Licensing agreement requires at least 4 parties.");
                        }
    
                        let agent_a_found = parties.iter().any(|party| party_is::<A>(&**party));
                        let agent_b_found = parties.iter().any(|party| party_is::<B>(&**party));
                        let generators_found = parties.iter().any(|party| party_is::<G>(&**party));
                        let h_bank_found = parties.iter().any(|party| party_is::<H>(&**party));
    
                        if !agent_a_found || !agent_b_found || !generators_found || !h_bank_found {
                            panic!("Parties do not match required types for LicensingAgreement");
                        }
                    },
                    TransactionLegalStructure::AccessAgreement { .. } => {
                        if parties.len() < 4 {
                            panic!("Access agreement requires at least 4 parties.");
                        }
    
                        let agent_a_found = parties.iter().any(|party| party_is::<A>(&**party));
                        let agent_b_found = parties.iter().any(|party| party_is::<B>(&**party));
                        let generators_found = parties.iter().any(|party| party_is::<G>(&**party));
                        let h_bank_found = parties.iter().any(|party| party_is::<H>(&**party));
    
                        if !agent_a_found || !agent_b_found || !generators_found || !h_bank_found {
                            panic!("Parties do not match required types for AccessAgreement");
                        }
                    },
                    TransactionLegalStructure::SubscriptionAgreement { .. } => {
                        if parties.len() < 4 {
                            panic!("Subscription agreement requires at least 4 parties.");
                        }
    
                        let agent_a_found = parties.iter().any(|party| party_is::<A>(&**party));
                        let agent_b_found = parties.iter().any(|party| party_is::<B>(&**party));
                        let generators_found = parties.iter().any(|party| party_is::<G>(&**party));
                        let h_bank_found = parties.iter().any(|party| party_is::<H>(&**party));
    
                        if !agent_a_found || !agent_b_found || !generators_found || !h_bank_found {
                            panic!("Parties do not match required types for SubscriptionAgreement");
                        }
                    },
                    TransactionLegalStructure::ConsortiumAgreement { .. } => {
                        if parties.len() < 4 {
                            panic!("Consortium agreement requires at least 4 parties.");
                        }
    
                        let agents_found = parties.iter().any(|party| party_is::<A>(&**party));
                        let generators_found = parties.iter().any(|party| party_is::<G>(&**party));
                        let h_bank_found = parties.iter().any(|party| party_is::<H>(&**party));

                        if !agents_found || !generators_found || !h_bank_found {
                            panic!("Parties do not match required types for ConsortiumAgreement");
                        }
                    },
                    TransactionLegalStructure::FundingAgreement { .. } => {
                        if parties.len() < 4 {
                            panic!("Funding agreement requires at least 4 parties.");
                        }
    
                        let agents_found = parties.iter().any(|party| party_is::<A>(&**party));
                        let funders_found = parties.iter().any(|party| party_is::<F>(&**party));
                        let generators_found = parties.iter().any(|party| party_is::<G>(&**party));
                        let h_bank_found = parties.iter().any(|party| party_is::<H>(&**party));
                        
                        if !agents_found || !funders_found || !generators_found || !h_bank_found {
                            panic!("Parties do not match required types for FundingAgreement");
                        }
                    },
                    TransactionLegalStructure::DataExchangeAgreement { .. } => {
                        if parties.len() < 4 {
                            panic!("Data exchange agreement requires at least 4 parties.");
                        }
    
                        let agent_a_found = parties.iter().any(|party| party_is::<A>(&**party));
                        let agent_b_found = parties.iter().any(|party| party_is::<B>(&**party));
                        let generators_found = parties.iter().any(|party| party_is::<G>(&**party));
                        let h_bank_found = parties.iter().any(|party| party_is::<H>(&**party));
    
                        if !agent_a_found || !agent_b_found || !generators_found || !h_bank_found {
                            panic!("Parties do not match required types for DataExchangeAgreement");
                        }
                    },
                }
            },
        }
    }
    

        // Other methods...
}
