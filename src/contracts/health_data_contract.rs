use std::fmt;
use std::collections::HashMap;
use time::OffsetDateTime;

use crate::contracts::structs_enums::*; 
use crate::persons::Individual;
use crate::persons::Corporation;


#[derive(Debug)]
pub struct ValidationError(String);

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Validation Error: {}", self.0)
    }
}

pub struct HealthDataContract {
    parties: Vec<Party>,
    agreement_type: ContractCategory,
    legal_framework: ContractLegalFramework,
    terms: Terms,
    generator_rate: Option<GeneratorRateSpecification>,
    residual_payments: Option<Residuals>,
    individual_contribution_level: Option<IndividualContributionLevel>,
    irb_required: bool,
    irb_approved: Option<bool>,
    pub contract_id: String,
    pub cohort_id: Option<String>,
    privacy_level: DataPrivacyLevel,
    individuals_map: HashMap<EntityId, Individual>,
    corporations_map: HashMap<EntityId, Corporation>, 
}

impl HealthDataContract {
    ///////////////////////////////////////////////////////
    //           Implement getter methods
    pub fn get_parties(&self) -> &Vec<Party> {
        &self.parties
    }

    pub fn get_privacy_level(&self) -> &DataPrivacyLevel {
        &self.privacy_level
    }

    pub fn get_contract_id(&self) -> &str {
        &self.contract_id
    }

    pub fn get_cohort_id(&self) -> Option<&str> {
        self.cohort_id.as_deref()
    }
    ///////////////////////////////////////////////////////
    


    pub fn new(
        parties: Vec<Party>,
        agreement_type: ContractCategory,
        legal_framework: ContractLegalFramework,
        terms: Terms,
        generator_rate: Option<GeneratorRateSpecification>,
        residual_payments: Option<Residuals>,
        individual_contribution_level: Option<IndividualContributionLevel>,
        irb_required: bool,
        irb_approved: Option<bool>,
        contract_id: String,
        cohort_id: Option<String>,
        privacy_level: DataPrivacyLevel,
        individuals_map: HashMap<EntityId, Individual>,
        corporations_map: HashMap<EntityId, Corporation>,
    ) -> Self {
        HealthDataContract {
            parties,
            agreement_type,
            legal_framework,
            terms,
            generator_rate,
            residual_payments,
            individual_contribution_level,
            irb_required,
            irb_approved,
            cohort_id,
            contract_id,
            privacy_level,
            individuals_map,
            corporations_map,
        }
    }

    pub fn add_parties(&mut self, new_parties: Vec<Party>) -> Result<(), ValidationError> {
        match &self.agreement_type {
            ContractCategory::TwoParty(two_party_type) => self.validate_two_party(two_party_type, &new_parties)?,
            ContractCategory::ThreePlusParty(three_plus_party_type) => self.validate_three_plus_party(three_plus_party_type, &new_parties)?,
        }
        self.parties.extend(new_parties);
        Ok(())
    }

    fn validate_two_party(&self, two_party_type: &TwoPartyLegalStructure, new_parties: &[Party]) -> Result<(), ValidationError> {
        if new_parties.len() != 2 {
            return Err(ValidationError("Two-party agreement requires exactly 2 parties.".into()));
        }
        match two_party_type {
            TwoPartyLegalStructure::Storage_or_Exchange(_) => {
                let agent_or_generator = new_parties.iter().any(|p| matches!(p, Party::DataOriginator(_) | Party::DataGenerator(_)));
                let h_bank = new_parties.iter().any(|p| matches!(p, Party::HBank(_)));
                if !agent_or_generator || !h_bank {
                    return Err(ValidationError("Parties do not match required types for Storage or Exchange Agreement".into()));
                }
            },
            TwoPartyLegalStructure::Donation(_) => {
                let donor = new_parties.iter().any(|p| matches!(p, Party::Donor(_)));
                let h_bank = new_parties.iter().any(|p| matches!(p, Party::HBank(_)));
                if !donor || !h_bank {
                    return Err(ValidationError("Parties do not match required types for Donation".into()));
                }
            },
            TwoPartyLegalStructure::Advertisement(_) => {
                let advertiser = new_parties.iter().any(|p| matches!(p, Party::Advertiser(_)));
                let h_bank = new_parties.iter().any(|p| matches!(p, Party::HBank(_)));
                if !advertiser || !h_bank {
                    return Err(ValidationError("Parties do not match required types for Advertisement".into()));
                }
            },
        }
        Ok(())
    }

    fn validate_three_plus_party(&self, three_plus_party_type: &TransactionLegalStructure, new_parties: &[Party]) -> Result<(), ValidationError> {
        if new_parties.len() < 3 {
            return Err(ValidationError("Three-plus party agreement requires at least 3 parties.".into()));
        }
        match three_plus_party_type {
            TransactionLegalStructure::ConsultAgreement { .. } => {
                let agent = new_parties.iter().any(|p| matches!(p, Party::DataOriginator(_) | Party::DataCustodian(_) | Party::DataRecipient(_)));
                let consultant = new_parties.iter().any(|p| matches!(p, Party::DataConsultant(_)));
                let generator = new_parties.iter().any(|p| matches!(p, Party::DataGenerator(_)));
                let h_bank = new_parties.iter().any(|p| matches!(p, Party::HBank(_)));
                if !agent || !consultant || !generator || !h_bank {
                    return Err(ValidationError("Parties do not match required types for ConsultAgreement".into()));
                }
            },
            TransactionLegalStructure::DirectSale { .. } |
            TransactionLegalStructure::PurchaseAgreement { .. } |
            TransactionLegalStructure::LicensingAgreement { .. } |
            TransactionLegalStructure::AccessAgreement { .. } |
            TransactionLegalStructure::SubscriptionAgreement { .. } => {
                let agent_a = new_parties.iter().any(|p| matches!(p, Party::DataOriginator(_) | Party::DataCustodian(_)));
                let agent_b = new_parties.iter().any(|p| matches!(p, Party::DataRecipient(_)));
                let generator = new_parties.iter().any(|p| matches!(p, Party::DataGenerator(_)));
                let h_bank = new_parties.iter().any(|p| matches!(p, Party::HBank(_)));
                if !agent_a || !agent_b || !generator || !h_bank {
                    return Err(ValidationError(format!("Parties do not match required types for {:?}", three_plus_party_type)));
                }
            },
            TransactionLegalStructure::ConsortiumAgreement { .. } => {
                let agents = new_parties.iter().any(|p| matches!(p, Party::DataOriginator(_) | Party::DataCustodian(_) | Party::DataRecipient(_)));
                let generator = new_parties.iter().any(|p| matches!(p, Party::DataGenerator(_)));
                let h_bank = new_parties.iter().any(|p| matches!(p, Party::HBank(_)));
                if !agents || !generator || !h_bank {
                    return Err(ValidationError("Parties do not match required types for ConsortiumAgreement".into()));
                }
            },
            TransactionLegalStructure::ParticipationAgreement { .. } => {
                let agents = new_parties.iter().any(|p| matches!(p, Party::DataOriginator(_) | Party::DataCustodian(_) | Party::DataRecipient(_)));
                let funders = new_parties.iter().any(|p| matches!(p, Party::Funder(_)));
                let generator = new_parties.iter().any(|p| matches!(p, Party::DataGenerator(_)));
                let h_bank = new_parties.iter().any(|p| matches!(p, Party::HBank(_)));
                if !agents || !funders || !generator || !h_bank {
                    return Err(ValidationError("Parties do not match required types for ParticipationAgreement".into()));
                }
            },
            TransactionLegalStructure::DataExchangeAgreement { .. } => {
                let agent_a = new_parties.iter().any(|p| matches!(p, Party::DataOriginator(_) | Party::DataCustodian(_)));
                let agent_b = new_parties.iter().any(|p| matches!(p, Party::DataRecipient(_)));
                let generator = new_parties.iter().any(|p| matches!(p, Party::DataGenerator(_)));
                let h_bank = new_parties.iter().any(|p| matches!(p, Party::HBank(_)));
                if !agent_a || !agent_b || !generator || !h_bank {
                    return Err(ValidationError("Parties do not match required types for DataExchangeAgreement".into()));
                }
            },
        }
        Ok(())
    }

    fn determine_whether_parties_have_generator(&self) -> bool {
        self.parties.iter().any(|party| matches!(party, Party::DataGenerator(_)))
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

    fn determine_whether_parties_have_data_originator(&self) -> bool {
        self.parties.iter().any(|party| matches!(party, Party::DataOriginator(_)))
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

    pub fn validate_individual_age_wrt_agency_privacy(&self) -> Result<(), ValidationError> {
        let current_date = OffsetDateTime::now_utc().date();
    
        for party in &self.parties {
            if let Party::DataOriginator(info) | Party::DataCustodian(info) | Party::DataRecipient(info) = party {
                let person_id = &info.entity_id;
                if let Some(individual) = self.individuals_map.get(person_id) {
                    let birth_date = individual.date_of_birth;
                    let age = current_date.year() - birth_date.year() -
                              if current_date.ordinal() < birth_date.ordinal() { 1 } else { 0 };
                    if age < 17 {
                        return Err(ValidationError(format!("Individual with ID {} is under 17 years old.", person_id.0)));
                    }
                } else {
                    return Err(ValidationError(format!("Person ID {} not found in individuals_map.", person_id.0)));
                }
            }
        }
        
        Ok(())
    }

    fn validate_residual_payees(&self) -> Result<(), ValidationError> {
        if let Some(residuals) = &self.residual_payments {
            for beneficiary in &residuals.Beneficiaries {
                if !self.parties.iter().any(|party| party == beneficiary) {
                    return Err(ValidationError(format!(
                        "Beneficiary {:?} is not a party to the contract.",
                        beneficiary
                    )));
                }
            }
        }
        Ok(())
    }

    pub fn validate_and_execute_contract(&self) -> Result<(), ValidationError> {
        self.validate_generator_rate_spec()?;
        self.validate_individual_contribution_level()?;
        self.validate_irb_requirement()?;
        self.validate_individual_age_wrt_agency_privacy()?;
        self.validate_residual_payees()?;

        // Here you would add the actual execution logic
        println!("Contract validated successfully. Ready for execution.");
        
        Ok(())
    }
}

impl PartialEq for HealthDataContract {
    fn eq(&self, other: &Self) -> bool {
        self.parties == other.parties
            && self.agreement_type == other.agreement_type
            && self.legal_framework == other.legal_framework
            && self.generator_rate == other.generator_rate
            && self.individual_contribution_level == other.individual_contribution_level
            && self.cohort_id == other.cohort_id
            && self.privacy_level == other.privacy_level
    }
}

impl Eq for HealthDataContract {}