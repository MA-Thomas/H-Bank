use std::fmt;
use std::any::{Any, TypeId};
use std::collections::HashMap;

use time::OffsetDateTime;

use std::marker::PhantomData;

use crate::contracts::structs_enums::{ContractCategory, ContractLegalFramework, DataCustodian, DataOriginator, DataRecipient, DonationLegalStructure, Donor, Funder, GeneratorRateSpecification, HBank, 
    IndividualContributionLevel, IsAdvertiser, IsAgent, IsConsultant, IsDonor, IsFunder, IsGenerator, IsHBank, IsOriginator, IsRecipient, Party, 
    StorageExchangeLegalStructure, TransactionLegalStructure, TwoPartyLegalStructure, 
    DataPrivacyLevel, Terms};

use crate::persons::*;


// Function to check if a party matches a type (useful for checking if party to be added is compatible with agreement_type)
fn party_is<T: Party>(party: &dyn Party) -> bool {
    TypeId::of::<T>() == party.type_id()
}

// Define a custom error type for validation errors
#[derive(Debug)]
pub struct ValidationError(String);
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Validation Error: {}", self.0)
    }
}

// ********* BRING IT ALL TOGETHER. USE THE DEFINED DATA TYPES TO DEFINE A HealthDataContract ********* 
/*
A : the data provider 
B : the data recipient 
C : the third-party service provider 
D : the donor (to HBank)
E : the advertiser (on Hbroker)
F : the funder (e.g., of research)
G : the data generator (e.g., a hospital)
O : the data originator (e.g., an individual person)
H : the HBank

Not all roles need to be present as parties to a contract, but all need to be part of the contract specification.
For now, cohort_id (update: and contract_id) is the only public field as it is the only field that needs to be used from other modules (i.e., cohorts/cohort_manager.rs)
*/
pub struct HealthDataContract<A, B, C, D, E, F, G, O, H> {
    parties: Vec<Box<dyn Party>>,
    agreement_type: ContractCategory,
    legal_framework: ContractLegalFramework,
    terms: Terms,
    generator_rate: Option<GeneratorRateSpecification>,
    individual_contribution_level: Option<IndividualContributionLevel>,
    irb_required: bool,
    irb_approved: Option<bool>,
    pub contract_id: String,
    pub cohort_id: Option<String>,
    privacy_level: DataPrivacyLevel,
    individuals_map: HashMap<String, Individual>,
    corporations_map: HashMap<String, Corporation>,
    _phantom: PhantomData<(A, B, C, D, E, F, G, O, H)>, // PhantomData to indicate unused type parameters (they will be used later for type checking)
}


impl<A, B, C, D, E, F, G, O, H> HealthDataContract<A, B, C, D, E, F, G, O, H> 
where 
    A: IsAgent + Party + 'static,
    B: IsAgent + IsRecipient + Party + 'static,
    C: IsConsultant + Party + 'static,
    D: IsDonor + Party + 'static,
    E: IsAdvertiser + Party + 'static,
    F: IsFunder + Party + 'static,
    G: IsGenerator + Party + 'static,
    O: IsAgent + IsOriginator + Party + 'static,
    H: IsHBank + Party + 'static,
{
    pub fn new(
        parties: Vec<Box<dyn Party>>,
        agreement_type: ContractCategory,
        legal_framework: ContractLegalFramework,
        terms: Terms,
        generator_rate: Option<GeneratorRateSpecification>,
        individual_contribution_level: Option<IndividualContributionLevel>,
        irb_required: bool,
        irb_approved: Option<bool>,
        contract_id: String,
        cohort_id: Option<String>,
        privacy_level: DataPrivacyLevel,
        individuals_map: HashMap<String, Individual>,
        corporations_map: HashMap<String, Corporation>,
    ) -> Self {

        // Assign default value if None
        HealthDataContract {
            parties,
            agreement_type,
            legal_framework,
            terms,
            generator_rate,
            individual_contribution_level,
            irb_required,
            irb_approved,
            cohort_id,
            contract_id,
            privacy_level,
            individuals_map,
            corporations_map,
            _phantom: PhantomData, // Initialize PhantomData without any value
        }
    }



    pub fn add_parties(&mut self, parties: Vec<Box<dyn Party>>) {
        match &self.agreement_type {
            ContractCategory::TwoParty(two_party_type) => {
                match two_party_type {
                    TwoPartyLegalStructure::Storage_or_Exchange(storage_type) => {
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
                    TwoPartyLegalStructure::Advertisement(ad_type) => {
                        if parties.len() != 2 {
                            panic!("Advertiser agreement requires exactly 2 parties.");
                        }
    
                        let advertiser_found = parties.iter().any(|party| party_is::<E>(&**party));
                        let h_bank_found = parties.iter().any(|party| party_is::<H>(&**party));
    
                        if !advertiser_found || !h_bank_found {
                            panic!("Parties do not match required types for Advertisement");
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
                    TransactionLegalStructure::ParticipationAgreement { .. } => {
                        if parties.len() < 4 {
                            panic!("Participation agreement requires at least 4 parties.");
                        }
    
                        let agents_found = parties.iter().any(|party| party_is::<A>(&**party));
                        let funders_found = parties.iter().any(|party| party_is::<F>(&**party));
                        let generators_found = parties.iter().any(|party| party_is::<G>(&**party));
                        let h_bank_found = parties.iter().any(|party| party_is::<H>(&**party));
                        
                        if !agents_found || !funders_found || !generators_found || !h_bank_found {
                            panic!("Parties do not match required types for ParticipationAgreement");
                        }
                    },
                    TransactionLegalStructure::DataExchangeAgreement { .. } => {
                        if parties.len() < 4 {
                            panic!("Data exchange agreement requires at least 4 parties.");
                        }
    
                        let agent_a_found = parties.iter().any(|party| party_is::<A>(&**party));
                        let agent_b_found = parties.iter().any(|party| party_is::<A>(&**party));
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
    

    // Method to validate generator_rate specification after parties are added.
    fn determine_whether_parties_have_generator(&self) -> bool {
        self.parties.iter().any(|party| party_is::<G>(&**party))
    }
    fn validate_generator_rate_spec(&self) -> Result<(), ValidationError> {
        let generator_present = self.determine_whether_parties_have_generator();
    
        match (generator_present, &self.generator_rate) {
            (false, None) => Ok(()),
            (false, Some(_)) => Err(ValidationError("No parties implement IsGenerator, but generator_rate is specified.".into())),
            (true, None) => Err(ValidationError("At least one party implements IsGenerator, but generator_rate is not specified.".into())),
            (true, Some(_)) => Ok(()),
        }
    }

    // Method to validate individual contribution (data only, data & participation, neither) after parties are added.
    fn determine_whether_parties_have_data_originator(&self) -> bool {
        self.parties.iter().any(|party| party_is::<O>(&**party))
    }
    fn validate_individual_contribution_level(&self) -> Result<(), ValidationError> {
        let originator_present = self.determine_whether_parties_have_data_originator();
    
        match (originator_present, &self.individual_contribution_level) {
            (false, None) => Ok(()),
            (false, Some(IndividualContributionLevel::DataOnly)) => Ok(()),
            (false, Some(IndividualContributionLevel::DataAndParticipation)) => Err(ValidationError("No parties implement IsOriginator, but individual_contribution_level is set to DataAndParticipation.".to_string())),
            (true, None) => Err(ValidationError("At least one party implements IsOriginator, but individual_contribution_level is not specified.".to_string())),
            (true, Some(IndividualContributionLevel::DataOnly)) => Ok(()),
            (true, Some(IndividualContributionLevel::DataAndParticipation)) => Ok(()),
        }
    }

    // Method to validate IRB if required.
    fn validate_irb_requirement(&self) -> Result<(), ValidationError> {
        if self.irb_required {
            match self.irb_approved {
                Some(true) => Ok(()),
                Some(false) => Err(ValidationError("IRB approval required but it was not granted.".to_string())),
                None => Err(ValidationError("IRB approval required but not specified.".to_string())),
            }
        } else {
            Ok(())
        }
    }

    // Method to verify each party's age related agency and privacy checks.
    pub fn validate_individual_age_wrt_agency_privacy(&self) -> Result<(), ValidationError> {
        let current_date = OffsetDateTime::now_utc().date();
    
        for party in &self.parties {
            let person_id = party.get_entity_id();
            
            if party_is::<DataOriginator>(&**party) || party_is::<DataCustodian>(&**party) {
                if let Some(individual) = self.individuals_map.get(person_id) {
                    let birth_date = individual.date_of_birth;
                    let age = current_date.year() - birth_date.year() -
                              if current_date.ordinal() < birth_date.ordinal() { 1 } else { 0 };
                    if age < 17 {
                        return Err(ValidationError(format!("Individual with ID {} is under 17 years old.", person_id)));
                    }
                } else {
                    return Err(ValidationError(format!("Person ID {} not found in individuals_map.", person_id)));
                }
            }
        }
        
        Ok(())
    }


    pub fn validate_and_execute_contract(&self) -> Result<(), ValidationError> {
        /*
        Calls methods beginning with 'validate_' before executing contract (execution method needs to be written).
         */
        match self.validate_generator_rate_spec() {
            Ok(_) => println!("Generator rate specification is valid."),
            Err(e) => eprintln!("Error: {}", e),
        }
        match self.validate_individual_contribution_level() {
            Ok(_) => println!("Individual contribution level is valid."),
            Err(e) => eprintln!("Error: {}", e),
        }
        match self.validate_irb_requirement() {
            Ok(_) => println!("IRB requirement is satisfied."),
            Err(e) => eprintln!("Error: {}", e),
        }
        match self.validate_individual_age_wrt_agency_privacy() {
            Ok(_) => println!("age related privacy requirements are satisfied."),
            Err(e) => eprintln!("Error: {}", e),
        }
        Ok(())

    }
        // Other methods...
}


// This allows us to compare if two HealthDataContract's are equal (e.g., in lib_cohorts.rs).
// Trait Inheritance: The Eq trait inherits from PartialEq, meaning any type that implements PartialEq correctly 
// (such that equality is reflexive, symmetric, and transitive) will automatically satisfy the requirements of Eq.
impl<A, B, C, D, E, F, G, O, H> Eq for HealthDataContract<A, B, C, D, E, F, G, O, H> {}


impl<A, B, C, D, E, F, G, O, H> PartialEq for HealthDataContract<A, B, C, D, E, F, G, O, H> {
    fn eq(&self, other: &Self) -> bool {
        self.parties.len() == other.parties.len()
            && self.parties.iter().zip(&other.parties).all(|(a, b)| party_eq(a, b))
            && self.agreement_type == other.agreement_type
            && self.legal_framework == other.legal_framework
            // && self.terms == other.terms
            && self.generator_rate == other.generator_rate
            && self.individual_contribution_level == other.individual_contribution_level
            && self.cohort_id == other.cohort_id
            && self.privacy_level == other.privacy_level
    }
}

// Helper function to compare Box<dyn Party> trait objects
fn party_eq(a: &Box<dyn Party>, b: &Box<dyn Party>) -> bool {
    // Implement the necessary logic to compare the trait objects.
    // The Party trait does implement the name so we can compare 
    // the entity IDs of the two objects that implement the Party trait.
    a.get_entity_id() == b.get_entity_id()
}

