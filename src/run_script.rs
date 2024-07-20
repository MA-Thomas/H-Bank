use std::collections::{HashMap, HashSet};
use std::fs::{File, remove_file, create_dir_all};
use std::io::BufReader;
use std::error::Error;




//////////////////////////////////////////////////////////////////////////////////////////
use crate::contracts;
use crate::persons;
use crate::cohorts;
/////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////

fn main() {

    /*
    CREATE DYNAMIC TRAIT OBJECTS.
    Initialize instances of various structs (DataOriginator, DataRecipient, etc.) implementing different traits 
    (IsAgent, IsConsultant, etc.) and create references to these instances as trait objects (&dyn Trait).

    A dynamic trait object (dyn Trait) is a reference to a trait (Trait) that can hold any instance of a type 
    that implements Trait. This allows different concrete types that implement the same trait to be stored and 
    used interchangeably, as long as they are referenced through a trait object.
     */
    let originator: &dyn IsAgent = &DataOriginator { name: "Marcus Thomas".to_string(), entity_id: "3015_AR_>#_2049".to_string() };
    let custodian: &dyn IsAgent = &DataCustodian { name: "Greenbaum Lab".to_string(), entity_id: "3015_AR_>#_2077".to_string() };
    let recipient: &dyn IsAgent = &DataRecipient { name: "Recipient Corp.".to_string(), entity_id: "3015_AR_>#_2066".to_string() };
    let consultant: &dyn IsConsultant = &DataConsultant { name: "consulting firm".to_string(), entity_id: "3015_AR_>#_2055".to_string() };
    let generator: &dyn IsGenerator = &DataGenerator { name: "The Hospital".to_string(), entity_id: "3015_AR_>#_2044".to_string() };
    let funder: &dyn IsFunder = &Funder { name: "Big Money Foundation".to_string(), entity_id: "3015_AR_>#_2033".to_string() };
    let advertiser: &dyn IsAdvertiser = &Advertiser { name: "Goog".to_string(), entity_id: "3015_AR_>#_2000".to_string() };
    let donor: &dyn IsDonor = &Donor { name: "Mystery Billionaire".to_string(), entity_id: "3015_AR_>#_2022".to_string() };
    let hbank: &dyn IsHBank = &HBank { name: "Reservatory".to_string(), entity_id: "3015_AR_>#_2011".to_string() };

    /*
    DETERMINE CONTRACT CATEGORY
     */
    let contract_category: ContractCategory =
        ContractCategory::ThreePlusParty(TransactionLegalStructure::DirectSale {
            // agent_a: Box::new(DataOriginator { name: originator.get_name().to_string() }) as Box<dyn IsAgent>,
            agent_a: Box::new(DataCustodian { name: custodian.get_name().to_string(), entity_id: custodian.get_entity_id().to_string() }) as Box<dyn IsAgent>,
            agent_b: Box::new(DataRecipient { name: recipient.get_name().to_string(), entity_id: recipient.get_entity_id().to_string() }) as Box<dyn IsAgent>,
            generators: vec![
                Box::new(DataGenerator { name: generator.get_name().to_string(), entity_id: generator.get_entity_id().to_string() }) as Box<dyn IsGenerator>,
                Box::new(DataGenerator { name: generator.get_name().to_string(), entity_id: generator.get_entity_id().to_string() }) as Box<dyn IsGenerator>,
                Box::new(DataGenerator { name: generator.get_name().to_string(), entity_id: generator.get_entity_id().to_string() }) as Box<dyn IsGenerator>,
            ],
            h_bank: Box::new(HBank { name: hbank.get_name().to_string(), entity_id: hbank.get_entity_id().to_string() }) as Box<dyn IsHBank>,
        });

    let legal_framework: ContractLegalFramework = ContractLegalFramework::CommonLaw;

    let generator_rate_spec = GeneratorRateSpecification::NotApplicable;

    let individual_contribution_level = IndividualContributionLevel::NotApplicable;

    let irb_required = false;
    let irb_approved = None; 

    // The type of cohort_id is Option<String>. This means the ownership of the string contained in the Option<> 
    // will be passed to the contract (rather than borrowed which might introduce lifetime issues).
    let cohort_id = Some("abcdXYZ31415".to_string());

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
            cohort_id,
        );

    // Define parties to add as Boxed references to dynamic trait objects
    let originator_party: Box<dyn Party> = Box::new(DataOriginator { name: originator.get_name().to_string(), entity_id: originator.get_entity_id().to_string() });
    let custodian_party: Box<dyn Party> = Box::new(DataCustodian { name: custodian.get_name().to_string(), entity_id: custodian.get_entity_id().to_string() });
    let recipient_party: Box<dyn Party> = Box::new(DataRecipient { name: recipient.get_name().to_string(), entity_id: recipient.get_entity_id().to_string() });
    let generator_party: Box<dyn Party> = Box::new(DataGenerator { name: generator.get_name().to_string(), entity_id: generator.get_entity_id().to_string() });
    let hbank_party: Box<dyn Party> = Box::new(HBank { name: hbank.get_name().to_string(), entity_id: hbank.get_entity_id().to_string() });
    
    let parties_to_add: Vec<Box<dyn Party>> = vec![
        custodian_party,
        recipient_party,
        generator_party,
        hbank_party,
    ];

    // Add parties using the add_parties method
    contract.add_parties(parties_to_add);

    // Adding additional terms to the contract
    contract.add_terms(" Additional terms: Confidentiality, data security.");

    // Validate contract before executing.
    contract.validate_and_execute_contract();
    match contract.validate_and_execute_contract() {
        Ok(_) => println!("Contract execute successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Execute contract ...

}