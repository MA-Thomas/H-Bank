// DEFINE TRAITS AND AGENT ROLES.
pub trait IsHBank {}
pub trait IsAgent {}
pub trait IsFunder {}
pub trait IsDonor {} 

pub struct HBank{
    name: String,
    // Add more attributes as needed
}

pub struct DataOriginator{
    name: String,
    // Add more attributes as needed
}

pub struct DataCustodian{
    name: String,
    // Add more attributes as needed
}

pub struct DataRecipient{
    name: String,
    // Add more attributes as needed
}

pub struct Funder{
    name: String,
    // Add more attributes as needed
}

pub struct Donor{
    name: String,
    // Add more attributes as needed
}



impl IsHBank for HBank {}
impl IsAgent for DataOriginator {}
impl IsAgent for DataCustodian {}
impl IsAgent for DataRecipient {}
impl IsFunder for Funder {}
impl IsDonor for Donor {}

// NEXT, DEFINE THE STORAGE AND TRANSACTION LEGAL STRUCTURES, INCLUDING THE AGREEMENT TYPES.
pub enum StorageLegalStructure<A: IsAgent, H: IsHBank> {
    StorageAgreement { agent: A, h_bank: H },
    ServiceAgreement { agent: A, h_bank: H },
}

pub enum DonationLegalStructure<D: IsDonor, H: IsHBank> {
    PhilanthropicAgreement { donor: D, h_bank: H },
    //Philantropic entity provides donations to HBank.
}

pub enum TransactionLegalStructure<A: IsAgent, B: IsAgent, F: IsFunder, H: IsHBank> {

    DirectSale { agent_a: A, agent_b: B, h_bank: H },
    // The originator sells their health data to a recipient for a one-time or recurring payments. Ownership of health data is transferred outright to the recipient, typically for a lump-sum payment or installment payments.

    PurchaseAgreement { agent_a: A, agent_b: B, h_bank: H },
    // A contractual agreement where ownership of health data is transferred from one agent to another agent in exchange for compensation. Specifies terms of sale, including price, transfer of ownership, and any warranties or liabilities related to the data.

    LicensingAgreement { agent_a: A, agent_b: B, h_bank: H }, 
    // The data recipient is granted specific rights to use the data under defined conditions, which can include the ability to modify, reproduce, distribute, and create derivative works based on the data. The license can be for a specific period or perpetual (but revokable). The agreement often specifies the allowed and prohibited uses of the data, which can include commercial exploitation. The recipient typically pays a fee, which can be a one-time payment or recurring. There might also be royalties based on the revenue generated from using the data.
    
    AccessAgreement { agent_a: A, agent_b: B, h_bank: H },
    // A more limited version of the LicensingAgreement. The recipient can view and use the data but often without the broader rights to modify, reproduce, distribute or commercialize the data.
    
    SubscriptionAgreement { agent_a: A, agent_b: B, h_bank: H },
    // Regular payments are made by the data recipient to continuously access updated health data. Involves recurring payments for ongoing access to potentially new or changing health data, often with provisions for updates and support. There may be subscription tiers, pricing structures, and access privileges.
    
    ResearchCollaboration { agent_a: A, agent_b: B, h_bank: H },
    // The data originator collaborates with the data recipient (typically a research institution) in a joint research project where access to health data is granted in exchange for funding or resources. Specifies the research objectives, funding arrangements, data sharing protocols, and intellectual property rights related to research outcomes.
    
    FundingAgreement { agent_a: A, funder: F, h_bank: H },
    // Research institutions or funding bodies provide grants to the data originator in exchange for access to health data for research purposes. Grants may fund specific research projects, with terms related to data access, use, publication rights, and compliance with regulatory requirements.
    
    DataExchangeAgreement { agent_a: A, agent_b: B, h_bank: H },
    // An agreement between two data agents where health data is exchanged. Defines the terms of data exchange, including data formats, protocols for data transmission, security measures, any reciprocal benefits and compensation.
}

// Define an enum to distinguish between Storage and Donation structures
pub enum TwoPartyLegalStructure<A: IsAgent, D: IsDonor, H: IsHBank> {
    Storage(StorageLegalStructure<A, H>),
    Donation(DonationLegalStructure<D, H>),
}

// DEFINE THE 'ContractCategory' ENUM WITH THE APPROPRIATE CONSTRAINTS.
pub enum ContractCategory<A: IsAgent, B: IsAgent, D: IsDonor, F: IsFunder, H: IsHBank> {
    TwoParty(TwoPartyLegalStructure<A, D, H>),
    ThreeParty(TransactionLegalStructure<A, B, F, H>),
}


pub enum ContractLegalFramework {
    UCC,        // For tangible goods, e.g., hardware, biological samples, etc.
    CommonLaw,  // For services, intellectual property. THIS IS THE DEFAULT FOR H-BANK.
}


/*                                *** SUMMARY ***
Traits: IsAgent and IsHBank are used to define the roles of agents and HBank.
Structs: StorageLegalStructure and TransactionLegalStructure include the various agreement types and enforce the type constraints.
Enum: ContractCategory uses generics to enforce that TwoParty contracts involve HBank and one other agent, and ThreeParty contracts involve HBank and two other agents.
*/

/*
NEXT, IMPLEMENT THE 'PARTY' TRAIT. Contracts will later be defined to include a vector of trait objects for the parties involved.
*/
// Trait for parties
pub trait Party {
    fn name(&self) -> &str;
    // Add more methods as needed
}

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
impl Party for DataRecipient {
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

