use std::collections::{HashMap, HashSet};
use std::fs::{File, remove_file, create_dir_all};
use std::io::BufReader;
use std::error::Error;


//////////////////////////////////////////////////////////////////////////////////////////
////////////   Declare modules (contents are in the associated files)       //////////////
// mod lib_io; 
mod lib_contracts; 
mod lib_structs_enums;

//////////////////////////////////////////////////////////////////////////////////////////
////////////   Bind these function names, data types to their full module paths         //////////////
// use lib_io::{}; 
use lib_contracts::{HealthDataContract};
use lib_structs_enums::{IsHBank, IsAgent, IsFunder, IsDonor, Party,
    HBank, DataOriginator, DataCustodian, DataRecipient, Funder, Donor,
    StorageLegalStructure, DonationLegalStructure, TransactionLegalStructure,
    TwoPartyLegalStructure, ContractCategory,
    ContractLegalFramework};

/////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////

fn main() {
    let originator = DataOriginator { name: "Marcus_Thomas".to_string() };
    let recipient = DataRecipient { name: "Recipient Corp.".to_string() };
    let funder = Funder { name: "Gates_Foundation".to_string() };
    let donor = Donor { name: "Mystery Billionaire".to_string() };
    let hbank = HealthBank { name: "Reservatory".to_string() };

    let contract_category = ContractCategory::ThreeParty(TransactionLegalStructure::DirectSale {
        agent_a: originator,
        agent_b: recipient,
        h_bank: hbank.clone(), // Cloning hbank for ownership in the contract
    });

    let legal_framework = ContractLegalFramework::CommonLaw;

    let mut contract = HealthDataContract::new(vec![], contract_category, legal_framework, "Sample terms. ".to_string());
    contract.add_party(Box::new(originator));
    contract.add_party(Box::new(recipient));
    contract.add_party(Box::new(funder));
    contract.add_party(Box::new(donor));
    contract.add_party(Box::new(hbank));

    // Adding additional terms to the contract
    contract.add_terms("Additional terms: Confidentiality, data security.");

    // Use contract...
}