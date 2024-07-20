use std::any::{Any, TypeId};
use std::fmt::Debug;



////////////////////////////////////////////////////////////////////////////////////////////
// DEFINE THE 9 ROLES (STRUCTS).
#[derive(Clone, Debug)]
pub struct HBank{
    pub name: String,
    pub entity_id: String,
    // Add more attributes as needed
}

#[derive(Clone, Debug)]
pub struct DataOriginator{
    pub name: String,
    pub entity_id: String,
    // Add more attributes as needed
}

#[derive(Clone, Debug)]
pub struct DataCustodian{
    pub name: String,
    pub entity_id: String,
    // Add more attributes as needed
}

#[derive(Clone, Debug)]
pub struct DataRecipient{
    pub name: String,
    pub entity_id: String,
    // Add more attributes as needed
}

#[derive(Clone, Debug)]
pub struct DataConsultant{
    pub name: String,
    pub entity_id: String,
    // Add more attributes as needed
}
#[derive(Clone, Debug)]
pub struct DataGenerator{
    pub name: String,
    pub entity_id: String,
    // Add more attributes as needed
}

#[derive(Clone, Debug)]
pub struct Funder{
    pub name: String,
    pub entity_id: String,
    // Add more attributes as needed
}

#[derive(Clone, Debug)]
pub struct Donor{
    pub name: String,
    pub entity_id: String,
    // Add more attributes as needed
}
#[derive(Clone, Debug)]
pub struct Advertiser{
    pub name: String,
    pub entity_id: String,
    // Add more attributes as needed
}

////////////////////////////////////////////////////////////////////////////////////////////



////////////////////////////////////////////////////////////////////////////////////////////
// DEFINE TRAITS APPLICABLE TO THE 8 ROLES.

pub trait Party: Any {
    fn get_name(&self) -> &str;
    fn get_entity_id(&self) -> &str;
}
// Make sure that the Party trait is a supertrait of all other relevant traits.
// These next lines ensure that each specific trait (IsHBank, IsAgent, IsOriginator, IsRecipient, etc.) 
// inherits from the Party trait, making the name and entity_id method available to them.
pub trait IsHBank: Party + Debug {}
pub trait IsAgent: Party + Debug {}
pub trait IsOriginator: Party + Debug {} 
pub trait IsRecipient: Party + Debug {} 
pub trait IsConsultant: Party + Debug {}
pub trait IsGenerator: Party + Debug {}
pub trait IsFunder: Party + Debug {}
pub trait IsDonor: Party + Debug {} 
pub trait IsAdvertiser: Party + Debug {} 
////////////////////////////////////////////////////////////////////////////////////////////


////////////////////////////////////////////////////////////////////////////////////////////
// IMPLEMENT THE PARTY TRAIT FOR ALL 8 ROLES.
impl Party for HBank {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_entity_id(&self) -> &str {
        &self.entity_id
    }
}
impl Party for DataOriginator {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_entity_id(&self) -> &str {
        &self.entity_id
    }
}
impl Party for DataCustodian {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_entity_id(&self) -> &str {
        &self.entity_id
    }
}
impl Party for DataRecipient {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_entity_id(&self) -> &str {
        &self.entity_id
    }
}
impl Party for DataConsultant {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_entity_id(&self) -> &str {
        &self.entity_id
    }
}
impl Party for DataGenerator {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_entity_id(&self) -> &str {
        &self.entity_id
    }
}
impl Party for Funder {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_entity_id(&self) -> &str {
        &self.entity_id
    }
}
impl Party for Donor {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_entity_id(&self) -> &str {
        &self.entity_id
    }
}
impl Party for Advertiser {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_entity_id(&self) -> &str {
        &self.entity_id
    }
}
////////////////////////////////////////////////////////////////////////////////////////////



////////////////////////////////////////////////////////////////////////////////////////////
// IMPLEMENT THE NON-PARTY TRAITS IN A CASE BY CASE MANNER.
impl IsHBank for HBank {}
impl IsOriginator for DataOriginator {} 
impl IsRecipient for DataRecipient {} 

// An agent is a person or entity (excluding HBank) that has some degree of agency over health data. 
impl IsAgent for DataOriginator {}  // The person from whom data was generated.
impl IsAgent for DataCustodian {}   // A person or entity with effective control over data (when different from the originator).
impl IsAgent for DataRecipient {}   // A person or entity to whom data access is temporarily granted.
impl IsAgent for DataConsultant {}  // A third party service provider, e.g., a person or entity engaged directly by the originator or custodian to temporarily access data in order to generate knowledge.

impl IsFunder for Funder {}
impl IsDonor for Donor {}
impl IsConsultant for DataConsultant {}
impl IsGenerator for DataGenerator {}
impl IsAdvertiser for Advertiser {}
////////////////////////////////////////////////////////////////////////////////////////////


////////////////////////////////////////////////////////////////////////////////////////////
// NEXT, IMPLEMENT PartialEq FOR ALL OF THE BOXED DYNAMIC TRAIT OBJECTS 
// SO WE CAN COMPARE TWO OBJECTS. ALSO IMPLEMENT PartialEq FOR AGREEMENT TYPES
// ( NECESSARY FOR lib_contracts.rs -> HealthDataContract.eq(_,_) ).
impl PartialEq for &Box<dyn IsHBank> {
    fn eq(&self, other: &Self) -> bool {
        // Dereference the boxes to access the underlying data
        // and compare based on their names or other properties.
        self.as_ref().get_entity_id() == other.as_ref().get_entity_id()
    }
}
impl PartialEq for &Box<dyn IsAgent> {
    fn eq(&self, other: &Self) -> bool {
        // Dereference the boxes to access the underlying data
        // and compare based on their names or other properties.
        self.as_ref().get_entity_id() == other.as_ref().get_entity_id()
    }
}
impl PartialEq for &Box<dyn IsOriginator> {
    fn eq(&self, other: &Self) -> bool {
        // Dereference the boxes to access the underlying data
        // and compare based on their names or other properties.
        self.as_ref().get_entity_id() == other.as_ref().get_entity_id()
    }
}
impl PartialEq for &Box<dyn IsRecipient> {
    fn eq(&self, other: &Self) -> bool {
        // Dereference the boxes to access the underlying data
        // and compare based on their names or other properties.
        self.as_ref().get_entity_id() == other.as_ref().get_entity_id()
    }
}
impl PartialEq for &Box<dyn IsGenerator> {
    fn eq(&self, other: &Self) -> bool {
        // Dereference the boxes to access the underlying data
        // and compare based on their names or other properties.
        self.as_ref().get_entity_id() == other.as_ref().get_entity_id()
    }
}
impl PartialEq for &Box<dyn IsConsultant> {
    fn eq(&self, other: &Self) -> bool {
        // Dereference the boxes to access the underlying data
        // and compare based on their names or other properties.
        self.as_ref().get_entity_id() == other.as_ref().get_entity_id()
    }
}
impl PartialEq for &Box<dyn IsFunder> {
    fn eq(&self, other: &Self) -> bool {
        // Dereference the boxes to access the underlying data
        // and compare based on their names or other properties.
        self.as_ref().get_entity_id() == other.as_ref().get_entity_id()
    }
}
impl PartialEq for &Box<dyn IsDonor> {
    fn eq(&self, other: &Self) -> bool {
        // Dereference the boxes to access the underlying data
        // and compare based on their names or other properties.
        self.as_ref().get_entity_id() == other.as_ref().get_entity_id()
    }
}
impl PartialEq for &Box<dyn IsAdvertiser> {
    fn eq(&self, other: &Self) -> bool {
        // Dereference the boxes to access the underlying data
        // and compare based on their names or other properties.
        self.as_ref().get_entity_id() == other.as_ref().get_entity_id()
    }
}

impl PartialEq for StorageLegalStructure {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&StorageLegalStructure::AgentStorageAgreement { .. }, &StorageLegalStructure::AgentStorageAgreement { .. }) => true,
            (&StorageLegalStructure::AgentServiceAgreement { .. }, &StorageLegalStructure::AgentServiceAgreement { .. }) => true,
            (&StorageLegalStructure::GeneratorStorageAgreement { .. }, &StorageLegalStructure::GeneratorStorageAgreement { .. }) => true,
            (&StorageLegalStructure::GeneratorServiceAgreement { .. }, &StorageLegalStructure::GeneratorServiceAgreement { .. }) => true,
            _ => false,
        }
    }
}
impl PartialEq for DonationLegalStructure {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&DonationLegalStructure::PhilanthropicAgreement { .. }, &DonationLegalStructure::PhilanthropicAgreement { .. }) => true,
            _ => false,
        }
    }
}
impl PartialEq for AdLegalStructure {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&AdLegalStructure::AdvertiserAgreement { .. }, &AdLegalStructure::AdvertiserAgreement { .. }) => true,
            _ => false,
        }
    }
}
impl PartialEq for TransactionLegalStructure {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&TransactionLegalStructure::ConsultAgreement { .. }, &TransactionLegalStructure::ConsultAgreement { .. }) => true,
            (&TransactionLegalStructure::DirectSale { .. }, &TransactionLegalStructure::DirectSale { .. }) => true,
            (&TransactionLegalStructure::PurchaseAgreement { .. }, &TransactionLegalStructure::PurchaseAgreement { .. }) => true,
            (&TransactionLegalStructure::LicensingAgreement { .. }, &TransactionLegalStructure::LicensingAgreement { .. }) => true,
            (&TransactionLegalStructure::AccessAgreement { .. }, &TransactionLegalStructure::AccessAgreement { .. }) => true,
            (&TransactionLegalStructure::SubscriptionAgreement { .. }, &TransactionLegalStructure::SubscriptionAgreement { .. }) => true,
            (&TransactionLegalStructure::ConsortiumAgreement { .. }, &TransactionLegalStructure::ConsortiumAgreement { .. }) => true,
            (&TransactionLegalStructure::ParticipationAgreement { .. }, &TransactionLegalStructure::ParticipationAgreement { .. }) => true,
            (&TransactionLegalStructure::DataExchangeAgreement { .. }, &TransactionLegalStructure::DataExchangeAgreement { .. }) => true,
            _ => false,
        }
    }
}
////////////////////////////////////////////////////////////////////////////////////////////



////////////////////////////////////////////////////////////////////////////////////////////
// NEXT, DEFINE THE STORAGE AND TRANSACTION LEGAL STRUCTURES.
#[derive(Debug)]
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

#[derive(Debug)]
pub enum DonationLegalStructure {
    PhilanthropicAgreement { 
        donor: Box<dyn IsDonor>, 
        h_bank: Box<dyn IsHBank>, 
    },
    //Philantropic entity provides donations to HBank.

}
#[derive(Debug)]
pub enum AdLegalStructure {
    AdvertiserAgreement { 
        advertiser: Box<dyn IsDonor>, 
        h_bank: Box<dyn IsHBank>, 
    },
    //Advertiser entity provides joins the HBroker platform.

}
#[derive(Debug)]
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

    ParticipationAgreement {
        agents: Vec<Box<dyn IsAgent>>,
        funders: Vec<Box<dyn IsFunder>>,
        generators: Vec<Box<dyn IsGenerator>>,
        h_bank: Box<dyn IsHBank>,
    },
    // Covers clinical trials, oberservational studies, etc. May require IRB approval.

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
#[derive(Debug, PartialEq)]
pub enum TwoPartyLegalStructure {
    Storage(StorageLegalStructure),
    Donation(DonationLegalStructure),
    Advertisement(AdLegalStructure),
}

// Define the 'ContractCategory' enum with the appropriate constraints
#[derive(Debug, PartialEq)]
pub enum ContractCategory {
    TwoParty(TwoPartyLegalStructure),
    ThreePlusParty(TransactionLegalStructure),
}


#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub enum GeneratorRateSpecification {
    // Both variants hold an f64 which is the percentage of contract compensation 
    // to be given to the data generator. The Usage/Knowledge rate depends on how many times 
    // the all or a portion of the data bundle has been previously transacted. 
    
    KnowledgeRate(f64),
    // Applies to first transaction of a datum. 
    UsageRate(f64),
    // Applies to subsequent transactions.
    NotApplicable,
    // Applies when no generator is part of the contract.
}

////////////////////////////////////////////////////////////////////////////////////////////
// NEXT, DEFINE ENUMS FOR INDIVIDUAL USER ASPECTS OF CONTRACTS
#[derive(Debug, PartialEq)]
pub enum IndividualContributionLevel {
    DataOnly,
    // Possible for any contract agreement, any party composition.
    DataAndParticipation,
    // A DataOriginator must be party to the contract.
    NotApplicable,
    // A DataOriginator must NOT be party to the contract.
}


pub struct PaymentInfo {

}

