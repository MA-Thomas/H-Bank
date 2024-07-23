use std::collections::{HashMap, HashSet};
use std::fs::{File, remove_file, create_dir_all};
use std::io::BufReader;
use std::error::Error;

use time::{Date, Month, PrimitiveDateTime};
use std::str::FromStr;

//////////////////////////////////////////////////////////////////////////////////////////
// Make available all functions and data structures from the library modules.
use h_bank::contracts::health_data_contract::*;  
use h_bank::contracts::structs_enums::*;   
use h_bank::contracts::cohorts::cohort_manager::*;
use h_bank::persons::{individual::*, Corporation};
/////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////

fn parse_dob_yyyy_mm_dd(date_str: &str) -> Result<Date, String> {
    // Split the date string into components
    let parts: Vec<&str> = date_str.split('-').collect();

    if parts.len() != 3 {
        return Err("Invalid date format".to_string());
    }

    // Parse the components into integers
    let year = i32::from_str(parts[0]).map_err(|_| "Invalid year")?;
    let month = u8::from_str(parts[1]).map_err(|_| "Invalid month")?;
    let day = u8::from_str(parts[2]).map_err(|_| "Invalid day")?;

    // Create a Date object
    Date::from_calendar_date(year, Month::try_from(month).map_err(|_| "Invalid month")?, day)
        .map_err(|e| format!("Failed to create date: {}", e))
}

fn main() {

    let date_str = "1900-01-01";
    let dob: Date = match parse_dob_yyyy_mm_dd(date_str) {
        Ok(date) => date,
        Err(e) => {
            eprintln!("Error parsing date: {}", e);
            return; // Exit the function if there is an error
        },
    };
    
    // CREATE INSTANCES OF INDIVIDUALS and CORPORATIONS
    let originator = Individual { 
        name: "Marcus Thomas".to_string(), 
        person_id: "3015_AR_>#_2049".to_string(), 
        date_of_birth: dob,
        hla_profile: None,
        blood_type: None,
    };
    
    let custodian = Corporation { 
        name: "Greenbaum Lab at MSK".to_string(), 
        person_id: "3015_AR_>#_2077".to_string(),
        tax_id: Some("12-2345665".to_string()),
    };
    
    let recipient = Corporation { 
        name: "Recipient Corp.".to_string(), 
        person_id: "3015_AR_>#_2066".to_string(), 
        tax_id: Some("12-2345666".to_string()),
    };
    
    let consultant = Corporation { 
        name: "consulting firm".to_string(), 
        person_id: "3015_AR_>#_2055".to_string(),
        tax_id: Some("12-2345667".to_string()),
    };
    
    let generator = Corporation { 
        name: "The Hospital".to_string(), 
        person_id: "3015_AR_>#_2044".to_string(), 
        tax_id: Some("12-2345668".to_string()),
    };
    
    let funder = Individual { 
        name: "Big Money Foundation".to_string(), 
        person_id: "3015_AR_>#_2033".to_string(),
        date_of_birth: dob,
        hla_profile: None,
        blood_type: None,
    };
    
    let advertiser = Corporation { 
        name: "Goog".to_string(), 
        person_id: "3015_AR_>#_2000".to_string(),
        tax_id: Some("12-2345669".to_string()),
    };
    
    let donor = Individual { 
        name: "Mystery Billionaire".to_string(), 
        person_id: "3015_AR_>#_2022".to_string(),
        date_of_birth: dob,
        hla_profile: None,
        blood_type: None,
    };
    
    let hbank = Corporation { 
        name: "Reservatory".to_string(), 
        person_id: "3015_AR_>#_2011".to_string(),
        tax_id: Some("15-3141500".to_string()),
    };

    /*
    DETERMINE CONTRACT CATEGORY
     */
    let contract_category: ContractCategory =
        ContractCategory::ThreePlusParty(TransactionLegalStructure::DirectSale {
            agent_a: Box::new(DataCustodian { name: custodian.get_name().to_string(), entity_id: custodian.get_person_id().to_string() }) as Box<dyn IsAgent>,
            agent_b: Box::new(DataRecipient { name: recipient.get_name().to_string(), entity_id: recipient.get_person_id().to_string() }) as Box<dyn IsAgent>,
            generators: vec![
                Box::new(DataGenerator { name: generator.get_name().to_string(), entity_id: generator.get_person_id().to_string() }) as Box<dyn IsGenerator>,
                Box::new(DataGenerator { name: generator.get_name().to_string(), entity_id: generator.get_person_id().to_string() }) as Box<dyn IsGenerator>,
                Box::new(DataGenerator { name: generator.get_name().to_string(), entity_id: generator.get_person_id().to_string() }) as Box<dyn IsGenerator>,
            ],
            h_bank: Box::new(HBank { name: hbank.get_name().to_string(), entity_id: hbank.get_person_id().to_string() }) as Box<dyn IsHBank>,
        });

    let legal_framework: ContractLegalFramework = ContractLegalFramework::CommonLaw;

    let generator_rate_spec = None; //Some(GeneratorRateSpecification::KnowledgeRate(0.1));
    let individual_contribution_level = None; //Some(IndividualContributionLevel::DataAndParticipation);
    let irb_required = false;
    let irb_approved = None; 
    let contract_id = "165XF9_PO".to_string();
    let cohort_id = Some("abcdXYZ31415".to_string());
    let privacy_level = DataPrivacyLevel::HIPPA_deidentified; 

    // Define parties to add as Boxed references to dynamic trait objects
    let originator_party: Box<dyn Party> = Box::new(DataOriginator { name: originator.get_name().to_string(), entity_id: originator.get_person_id().to_string() });
    let custodian_party: Box<dyn Party> = Box::new(DataCustodian { name: custodian.get_name().to_string(), entity_id: custodian.get_person_id().to_string() });
    let recipient_party: Box<dyn Party> = Box::new(DataRecipient { name: recipient.get_name().to_string(), entity_id: recipient.get_person_id().to_string() });
    let generator_party: Box<dyn Party> = Box::new(DataGenerator { name: generator.get_name().to_string(), entity_id: generator.get_person_id().to_string() });
    let hbank_party: Box<dyn Party> = Box::new(HBank { name: hbank.get_name().to_string(), entity_id: hbank.get_person_id().to_string() });
    
    
    let parties_to_add: Vec<Box<dyn Party>> = vec![
        custodian_party,
        recipient_party,
        generator_party,
        hbank_party,
    ];

    /*
    Now it is ok to use (move, rather than borrow) the Individuals and Corporations
    by placing them in a Hashmap and sending them to HealthDataContract::new().
     */
    // Create a HashMap from person_id to Individual.
    let mut individuals_map: HashMap<String, Individual> = HashMap::new();
    individuals_map.insert(originator.person_id.clone(), originator);
    individuals_map.insert(donor.person_id.clone(), donor);
    individuals_map.insert(funder.person_id.clone(), funder);

    // Create a HashMap from person_id to Corporation. 
    let mut corporations_map: HashMap<String, Corporation> = HashMap::new();
    corporations_map.insert(custodian.person_id.clone(), custodian);
    corporations_map.insert(recipient.person_id.clone(), recipient);
    corporations_map.insert(consultant.person_id.clone(), consultant);
    corporations_map.insert(generator.person_id.clone(), generator);
    corporations_map.insert(advertiser.person_id.clone(), advertiser);
    corporations_map.insert(hbank.person_id.clone(), hbank);

    // Explicitly annotate the type of `contract` to resolve type inference issues
    let mut contract: HealthDataContract<DataCustodian, DataRecipient, DataConsultant, Donor, Advertiser, Funder, DataGenerator, DataOriginator, HBank> =
    HealthDataContract::new(
        vec![],
        contract_category,
        legal_framework,
        "Sample terms.".to_string(),
        generator_rate_spec,
        individual_contribution_level,
        irb_required,
        irb_approved,
        contract_id,
        cohort_id,
        privacy_level,
        individuals_map,
        corporations_map,
    );

    // Add parties using the add_parties method
    contract.add_parties(parties_to_add);

    // Adding additional terms to the contract
    contract.add_terms(" Additional terms: Confidentiality, data security.");

    // Validate contract before executing.
    contract.validate_and_execute_contract();
    match contract.validate_and_execute_contract() {
        Ok(_) => println!("Contract executed successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Execute contract ...
}
