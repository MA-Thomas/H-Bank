use std::collections::{HashMap, HashSet};
use std::fs::{File, remove_file, create_dir_all};
use std::io::BufReader;
use std::error::Error;


//////////////////////////////////////////////////////////////////////////////////////////
////////////   Declare modules (contents are in the associated files)       //////////////
// mod lib_io; 
mod lib_contracts; 
mod lib_contract_structs_enums;

//////////////////////////////////////////////////////////////////////////////////////////
////////////   Bind these function names, data types to their full module paths         //////////////
// use lib_io::{}; 
use lib_contracts::{HealthDataContract};
use lib_contract_structs_enums::{ContractCategory, ContractLegalFramework, DataConsultant, DataCustodian, DataGenerator, DataOriginator, DataRecipient, DonationLegalStructure, Donor, Funder, GeneratorRateSpecification, HBank, IsAgent, IsConsultant, IsDonor, IsFunder, IsGenerator, IsHBank, Party, StorageLegalStructure, TransactionLegalStructure, TwoPartyLegalStructure};

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
    let originator: &dyn IsAgent = &DataOriginator { name: "Marcus Thomas".to_string() };
    let custodian: &dyn IsAgent = &DataCustodian { name: "Greenbaum Lab".to_string() };
    let recipient: &dyn IsAgent = &DataRecipient { name: "Recipient Corp.".to_string() };
    let consultant: &dyn IsConsultant = &DataConsultant { name: "consulting firm".to_string() };
    let generator: &dyn IsGenerator = &DataGenerator { name: "The Hospital".to_string() };
    let funder: &dyn IsFunder = &Funder { name: "Big Money Foundation".to_string() };
    let donor: &dyn IsDonor = &Donor { name: "Mystery Billionaire".to_string() };
    let hbank: &dyn IsHBank = &HBank { name: "Reservatory".to_string() };

    /*
    DETERMINE CONTRACT CATEGORY
     */
    let contract_category: ContractCategory =
        ContractCategory::ThreePlusParty(TransactionLegalStructure::DirectSale {
            // agent_a: Box::new(DataOriginator { name: originator.get_name().to_string() }) as Box<dyn IsAgent>,
            agent_a: Box::new(DataCustodian { name: custodian.get_name().to_string() }) as Box<dyn IsAgent>,
            agent_b: Box::new(DataRecipient { name: recipient.get_name().to_string() }) as Box<dyn IsAgent>,
            generators: vec![
                Box::new(DataGenerator { name: generator.get_name().to_string() }) as Box<dyn IsGenerator>,
                Box::new(DataGenerator { name: generator.get_name().to_string() }) as Box<dyn IsGenerator>,
                Box::new(DataGenerator { name: generator.get_name().to_string() }) as Box<dyn IsGenerator>,
            ],
            h_bank: Box::new(HBank { name: hbank.get_name().to_string() }) as Box<dyn IsHBank>,
        });

    let legal_framework: ContractLegalFramework = ContractLegalFramework::CommonLaw;

    let generator_rate_spec = GeneratorRateSpecification::NotApplicable;
    // let generator_rate_spec = GeneratorRateSpecification::KnowledgeRate(0.05);
    // let generator_rate_spec = GeneratorRateSpecification::UsageRate(0.01);

    
    // Explicitly annotate the type of `contract` to resolve type inference issues
    let mut contract: HealthDataContract<DataCustodian, DataRecipient, DataConsultant, Donor, Funder, DataGenerator, HBank> =
        HealthDataContract::new(
            vec![],
            contract_category,
            legal_framework,
            "Sample terms.".to_string(),
            generator_rate_spec,
        );

    // Define parties to add as Boxed references to dynamic trait objects
    let originator_party: Box<dyn Party> = Box::new(DataOriginator { name: originator.get_name().to_string() });
    let custodian_party: Box<dyn Party> = Box::new(DataCustodian { name: custodian.get_name().to_string() });
    let recipient_party: Box<dyn Party> = Box::new(DataRecipient { name: recipient.get_name().to_string() });
    let generator_party: Box<dyn Party> = Box::new(DataGenerator { name: generator.get_name().to_string() });
    let hbank_party: Box<dyn Party> = Box::new(HBank { name: hbank.get_name().to_string() });
    
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

    // Make sure the Generator Rate Specification (knowledge rate, usage rate, N/A) 
    // is set appropriately based on the parties to the contract.
    contract.validate_generator_rate_spec();
    match contract.validate_generator_rate_spec() {
        Ok(_) => println!("Generator rate specification is valid."),
        Err(e) => eprintln!("Error: {}", e),
    }


    // Use contract...
}