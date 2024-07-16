use std::any::{Any, TypeId};


////////////////////////////////////////////////////////////////////////////////////////////
// DEFINE THE 8 ROLES (STRUCTS).
#[derive(Clone)]
pub struct HBank{
    pub name: String,
    // Add more attributes as needed
}

#[derive(Clone)]
pub struct DataOriginator{
    pub name: String,
    // Add more attributes as needed
}

#[derive(Clone)]
pub struct DataCustodian{
    pub name: String,
    // Add more attributes as needed
}

#[derive(Clone)]
pub struct DataRecipient{
    pub name: String,
    // Add more attributes as needed
}

#[derive(Clone)]
pub struct DataConsultant{
    pub name: String,
    // Add more attributes as needed
}
#[derive(Clone)]
pub struct DataGenerator{
    pub name: String,
    // Add more attributes as needed
}

#[derive(Clone)]
pub struct Funder{
    pub name: String,
    // Add more attributes as needed
}

#[derive(Clone)]
pub struct Donor{
    pub name: String,
    // Add more attributes as needed
}

////////////////////////////////////////////////////////////////////////////////////////////



////////////////////////////////////////////////////////////////////////////////////////////
// DEFINE TRAITS APPLICABLE TO THE 8 ROLES.
pub trait Party: Any {
    fn name(&self) -> &str;
}
pub trait IsHBank {
    fn get_name(&self) -> &str;
}
pub trait IsAgent {
    fn get_name(&self) -> &str;
} 
pub trait IsConsultant {
    fn get_name(&self) -> &str;
}
pub trait IsGenerator {
    fn get_name(&self) -> &str;
}
pub trait IsFunder {
    fn get_name(&self) -> &str;
}
pub trait IsDonor {
    fn get_name(&self) -> &str;
} 

////////////////////////////////////////////////////////////////////////////////////////////


////////////////////////////////////////////////////////////////////////////////////////////
// IMPLEMENT THE PARTY TRAIT FOR ALL 8 ROLES.
impl Party for HBank {
    fn name(&self) -> &str {
        &self.name
    }
}
impl Party for DataOriginator {
    fn name(&self) -> &str {
        &self.name
    }
}
impl Party for DataCustodian {
    fn name(&self) -> &str {
        &self.name
    }
}
impl Party for DataRecipient {
    fn name(&self) -> &str {
        &self.name
    }
}
impl Party for DataConsultant {
    fn name(&self) -> &str {
        &self.name
    }
}
impl Party for DataGenerator {
    fn name(&self) -> &str {
        &self.name
    }
}
impl Party for Funder {
    fn name(&self) -> &str {
        &self.name
    }
}
impl Party for Donor {
    fn name(&self) -> &str {
        &self.name
    }
}
////////////////////////////////////////////////////////////////////////////////////////////



////////////////////////////////////////////////////////////////////////////////////////////
// IMPLEMENT THE NON-PARTY TRAITS IN A CASE BY CASE MANNER.
impl IsHBank for HBank {
    fn get_name(&self) -> &str {
        &self.name
    }
}

// An agent is a person or entity (excluding HBank) that has some degree of agency over health data. 
impl IsAgent for DataOriginator {
    fn get_name(&self) -> &str {
        &self.name
    }
}  // The person from whom data was generated.
impl IsAgent for DataCustodian {
    fn get_name(&self) -> &str {
        &self.name
    }
}   // A person or entity with effective control over data (when different from the originator).
impl IsAgent for DataRecipient {
    fn get_name(&self) -> &str {
        &self.name
    }
}   // A person or entity to whom data access is temporarily granted.
impl IsAgent for DataConsultant {
    fn get_name(&self) -> &str {
        &self.name
    }
}  // A third party service provider, e.g., a person or entity engaged directly by the originator or custodian to temporarily access data in order to generate knowledge.



impl IsFunder for Funder {
    fn get_name(&self) -> &str {
        &self.name
    }
}
impl IsDonor for Donor {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl IsConsultant for DataConsultant {
    fn get_name(&self) -> &str {
        &self.name
    }
}
impl IsGenerator for DataGenerator {
    fn get_name(&self) -> &str {
        &self.name
    }
}
////////////////////////////////////////////////////////////////////////////////////////////







////////////////////////////////////////////////////////////////////////////////////////////
// NEXT, DEFINE THE STORAGE AND TRANSACTION LEGAL STRUCTURES.
pub enum StorageLegalStructure {
    AgentStorageAgreement { 
        agent: Box<dyn IsAgent>, 
        h_bank: Box<dyn IsHBank>, 
    },
    //Storage agreement would be used by an agent to store their data.
    AgentServiceAgreement { 
        agent: Box<dyn IsAgent>, 
        h_bank: Box<dyn IsHBank>, 
    },
    //Service agreement would be used when agent seeks to gain access to the HBroker platform.
    GeneratorStorageAgreement { 
        generator: Box<dyn IsGenerator>, 
        h_bank: Box<dyn IsHBank>, 
    },
    //Storage agreement would be used by a data generator (e.g., hospital) to store their data.
    GeneratorServiceAgreement { 
        generator: Box<dyn IsGenerator>, 
        h_bank: Box<dyn IsHBank>, 
    },
    //Service agreement would be used when data generator seeks to gain access to the HBroker platform.
}
pub enum DonationLegalStructure {
    PhilanthropicAgreement { 
        donor: Box<dyn IsDonor>, 
        h_bank: Box<dyn IsHBank>, 
    },
    //Philantropic entity provides donations to HBank.
}

pub enum TransactionLegalStructure {

    ConsultAgreement {
        agent_a: Box<dyn IsAgent>,
        consultant_c: Box<dyn IsConsultant>,
        h_bank: Box<dyn IsHBank>,
    },
    // The agent engages a third-party service provider (consultant) to access health data and generate knowledge for the agent(s).

    DirectSale {
        agent_a: Box<dyn IsAgent>,
        agent_b: Box<dyn IsAgent>,
        generators: Vec<Box<dyn IsGenerator>>,
        h_bank: Box<dyn IsHBank>,
    },
    // The agent(s) sell health data to recipient(s) for a one-time or recurring payment. Ownership of health data is transferred outright to the recipient(s), typically for a lump-sum payment or installment payments.

    PurchaseAgreement {
        agent_a: Box<dyn IsAgent>,
        agent_b: Box<dyn IsAgent>,
        generators: Vec<Box<dyn IsGenerator>>,
        h_bank: Box<dyn IsHBank>,
    },
    // A contractual agreement where ownership of health data is transferred from one agent to another agent in exchange for compensation. Specifies terms of sale, including price, transfer of ownership, and any warranties or liabilities related to the data.

    LicensingAgreement {
        agent_a: Box<dyn IsAgent>,
        agent_b: Box<dyn IsAgent>,
        generators: Vec<Box<dyn IsGenerator>>,
        h_bank: Box<dyn IsHBank>,
    },
    // The data recipient is granted specific rights to use the data under defined conditions, which can include the ability to modify, reproduce, distribute, and create derivative works based on the data. The license can be for a specific period or perpetual (but revocable). The agreement often specifies the allowed and prohibited uses of the data, which can include commercial exploitation. The recipient typically pays a fee, which can be a one-time payment or recurring. There might also be royalties based on the revenue generated from using the data.

    AccessAgreement {
        agent_a: Box<dyn IsAgent>,
        agent_b: Box<dyn IsAgent>,
        generators: Vec<Box<dyn IsGenerator>>,
        h_bank: Box<dyn IsHBank>,
    },
    // A more limited version of the LicensingAgreement. The recipient can view and use the data but often without the broader rights to modify, reproduce, distribute, or commercialize the data.

    SubscriptionAgreement {
        agent_a: Box<dyn IsAgent>,
        agent_b: Box<dyn IsAgent>,
        generators: Vec<Box<dyn IsGenerator>>,
        h_bank: Box<dyn IsHBank>,
    },
    // Regular payments are made by the data recipient to continuously access updated health data. Involves recurring payments for ongoing access to potentially new or changing health data, often with provisions for updates and support. There may be subscription tiers, pricing structures, and access privileges.

    ConsortiumAgreement {
        agents: Vec<Box<dyn IsAgent>>,
        generators: Vec<Box<dyn IsGenerator>>,
        h_bank: Box<dyn IsHBank>,
    },
    // Multiple agents collaborate (typically at least one is a research institution) in a joint research project where access to health data is granted in exchange for funding or resources. Specifies the research objectives, funding arrangements, data sharing protocols, and intellectual property rights related to research outcomes.

    FundingAgreement {
        agents: Vec<Box<dyn IsAgent>>,
        funders: Vec<Box<dyn IsFunder>>,
        generators: Vec<Box<dyn IsGenerator>>,
        h_bank: Box<dyn IsHBank>,
    },
    // Research institutions or funding bodies provide grants to the agent in exchange for access to health data for research purposes. Grants may fund specific research projects, with terms related to data access, use, publication rights, and compliance with regulatory requirements.

    DataExchangeAgreement {
        agents_a: Vec<Box<dyn IsAgent>>,
        agents_b: Vec<Box<dyn IsAgent>>,
        generators: Vec<Box<dyn IsGenerator>>,
        h_bank: Box<dyn IsHBank>,
    },
    // An agreement between two sets of data agents where health data is exchanged. Defines the terms of data exchange, including data formats, protocols for data transmission, security measures, and any reciprocal benefits. There is no compensation (aside from the HBroker processing fee).
}
////////////////////////////////////////////////////////////////////////////////////////////


////////////////////////////////////////////////////////////////////////////////////////////
// Define an enum to distinguish between Storage and Donation structures
pub enum TwoPartyLegalStructure {
    Storage(StorageLegalStructure),
    Donation(DonationLegalStructure),
}

// Define the 'ContractCategory' enum with the appropriate constraints
pub enum ContractCategory {
    TwoParty(TwoPartyLegalStructure),
    ThreePlusParty(TransactionLegalStructure),
}



pub enum ContractLegalFramework {
    UCC,        // For tangible goods, e.g., hardware, biological samples, etc.
    CommonLaw,  // For services, intellectual property. THIS IS THE DEFAULT FOR H-BANK.
}


/*                                *** SUMMARY ***
Traits: IsAgent and IsHBank are used to define the roles of agents and HBank.
Structs: StorageLegalStructure and TransactionLegalStructure include the various agreement types and enforce the type constraints.
Enum: ContractCategory uses generics to enforce that TwoParty contracts involve HBank and one other agent, and ThreePlusParty contracts involve HBank and two other agents.
*/

////////////////////////////////////////////////////////////////////////////////////////////
// NEXT, DEFINE ENUMS FOR FINANCIAL ASPECTS OF CONTRACTS
pub enum GeneratorRateSpecification {
    // Both variants hold an f64 which is the percentage of contract compensation 
    // to be given to the data generator.
    
    KnowledgeRate(f64),
    // Applies to first transaction of a datum. 
    UsageRate(f64),
    // Applies to subsequent transactions
    NotApplicable,
}