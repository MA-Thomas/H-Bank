use std::collections::HashMap;
use time::{Date, Month};

use h_bank::contracts::health_data_contract::*;
use h_bank::contracts::structs_enums::*;
use h_bank::persons::{Individual, Corporation};

fn main() {
    // Create individuals
    let mut individual_originator = Individual::new(
        "John Doe".to_string(),
        EntityId("I-O123".to_string()),
        Date::from_calendar_date(1980, Month::January, 1).unwrap()
    );
    individual_originator.add_hla_profile(vec!["A02:01", "B07:02", "C01:02"]);
    individual_originator.add_blood_type("A+");

    let mut individual_donor = Individual::new(
        "Jane Smith".to_string(),
        EntityId("I-D123".to_string()),
        Date::from_calendar_date(1985, Month::May, 15).unwrap()
    );
    individual_donor.add_hla_profile(vec!["A01:01", "B08:01", "C07:01"]);
    individual_donor.add_blood_type("O-");

    let individual_funder = Individual::new(
        "Bob Johnson".to_string(),
        EntityId("I-F123".to_string()),
        Date::from_calendar_date(1975, Month::December, 10).unwrap()
    );

    // Create corporations
    let mut corp_custodian = Corporation::new(
        "Data Custodian Inc.".to_string(),
        EntityId("C-C123".to_string())
    );
    corp_custodian.add_tax_id("12-3456789");

    let mut corp_recipient = Corporation::new(
        "Data Recipient Corp.".to_string(),
        EntityId("C-R123".to_string())
    );
    corp_recipient.add_tax_id("98-7654321");

    let corp_consultant = Corporation::new(
        "Consultant LLC".to_string(),
        EntityId("C-CO123".to_string())
    );

    let corp_generator = Corporation::new(
        "Data Generator Co.".to_string(),
        EntityId("C-G123".to_string())
    );

    let corp_advertiser = Corporation::new(
        "Ad Agency Inc.".to_string(),
        EntityId("C-A123".to_string())
    );

    let corp_hbank = Corporation::new(
        "Health Bank".to_string(),
        EntityId("C-HB123".to_string())
    );

    // Create contract parties
    let originator = Party::DataOriginator(PartyInfo { name: individual_originator.get_name().to_string(), entity_id: individual_originator.get_person_id().clone() });
    let donor = Party::Donor(PartyInfo { name: individual_donor.get_name().to_string(), entity_id: individual_donor.get_person_id().clone() });
    let funder = Party::Funder(PartyInfo { name: individual_funder.get_name().to_string(), entity_id: individual_funder.get_person_id().clone() });
    let custodian = Party::DataCustodian(PartyInfo { name: corp_custodian.get_name().to_string(), entity_id: corp_custodian.get_person_id().clone() });
    let recipient = Party::DataRecipient(PartyInfo { name: corp_recipient.get_name().to_string(), entity_id: corp_recipient.get_person_id().clone() });
    let consultant = Party::DataConsultant(PartyInfo { name: corp_consultant.get_name().to_string(), entity_id: corp_consultant.get_person_id().clone() });
    let generator = Party::DataGenerator(PartyInfo { name: corp_generator.get_name().to_string(), entity_id: corp_generator.get_person_id().clone() });
    let advertiser = Party::Advertiser(PartyInfo { name: corp_advertiser.get_name().to_string(), entity_id: corp_advertiser.get_person_id().clone() });
    let hbank = Party::HBank(PartyInfo { name: corp_hbank.get_name().to_string(), entity_id: corp_hbank.get_person_id().clone() });

    // Define contract details
    let contract_category = ContractCategory::TwoParty(TwoPartyLegalStructure::Storage_or_Exchange(
        StorageExchangeLegalStructure::AgentStorageAgreement { 
            agent: custodian.clone(), 
            h_bank: hbank.clone() 
        }
    ));
    let legal_framework = ContractLegalFramework::CommonLaw;
    let generator_rate_spec = GeneratorRateSpecification::KnowledgeRate(0.05);
    let individual_contribution_level = IndividualContributionLevel::DataOnly;
    let contract_id = "C-001".to_string();
    let cohort_id = Some("CH-001".to_string());
    let privacy_level = DataPrivacyLevel::HIPPA_deidentified;

    // let parties_to_add = vec![
    //     originator.clone(), donor.clone(), funder.clone(), custodian.clone(),
    //     recipient.clone(), consultant.clone(), generator.clone(), advertiser.clone(), hbank.clone(),
    // ];
    let parties_to_add = vec![
        originator.clone(), hbank.clone(),
    ];

    // Create HashMaps for individuals and corporations
    let mut individuals_map: HashMap<EntityId, Individual> = HashMap::new();
    individuals_map.insert(individual_originator.get_person_id().clone(), individual_originator);
    individuals_map.insert(individual_donor.get_person_id().clone(), individual_donor);
    individuals_map.insert(individual_funder.get_person_id().clone(), individual_funder);

    let mut corporations_map: HashMap<EntityId, Corporation> = HashMap::new();
    corporations_map.insert(corp_custodian.get_person_id().clone(), corp_custodian);
    corporations_map.insert(corp_recipient.get_person_id().clone(), corp_recipient);
    corporations_map.insert(corp_consultant.get_person_id().clone(), corp_consultant);
    corporations_map.insert(corp_generator.get_person_id().clone(), corp_generator);
    corporations_map.insert(corp_advertiser.get_person_id().clone(), corp_advertiser);
    corporations_map.insert(corp_hbank.get_person_id().clone(), corp_hbank);

    // Create default Terms of the contract
    let contract_terms = Terms::default();

    // Create the contract
    let mut contract = HealthDataContract::new(
        vec![],
        contract_category,
        legal_framework,
        contract_terms,
        None, //Some(generator_rate_spec),
        Some(individual_contribution_level),
        true,
        Some(true),
        contract_id,
        cohort_id,
        privacy_level,
        individuals_map,
        corporations_map,
    );

    // Add parties
    if let Err(e) = contract.add_parties(parties_to_add) {
        eprintln!("Error adding parties: {}", e);
        return;
    }

    // Validate and execute contract
    if let Err(e) = contract.validate_and_execute_contract() {
        eprintln!("Contract validation failed: {}", e);
    } else {
        println!("Contract validated and executed successfully.");
    }
}