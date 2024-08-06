use std::any::{Any, TypeId};
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityId(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct Party {
    pub name: String,
    pub entity_id: EntityId,
}

pub trait IsParty: Any + Debug {
    fn as_party(&self) -> &Party;
}

pub trait IsHBank: IsParty {}
pub trait IsAgent: IsParty {}
pub trait IsOriginator: IsParty {}
pub trait IsRecipient: IsParty {}
pub trait IsConsultant: IsParty {}
pub trait IsGenerator: IsParty {}
pub trait IsFunder: IsParty {}
pub trait IsDonor: IsParty {}
pub trait IsAdvertiser: IsParty {}

macro_rules! implement_party {
    ($struct_name:ident, $($trait_name:ident),+) => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $struct_name(Party);

        impl $struct_name {
            pub fn new(name: String, entity_id: EntityId) -> Self {
                Self(Party { name, entity_id })
            }
        }

        impl IsParty for $struct_name {
            fn as_party(&self) -> &Party {
                &self.0
            }
        }

        $(impl $trait_name for $struct_name {})+
    };
}

implement_party!(HBank, IsHBank);
implement_party!(DataOriginator, IsAgent, IsOriginator);
implement_party!(DataCustodian, IsAgent);
implement_party!(DataRecipient, IsAgent, IsRecipient);
implement_party!(DataConsultant, IsConsultant);
implement_party!(DataGenerator, IsGenerator);
implement_party!(Funder, IsFunder);
implement_party!(Donor, IsDonor);
implement_party!(Advertiser, IsAdvertiser);

#[derive(Debug, Clone, PartialEq)]
pub enum StorageExchangeLegalStructure {
    AgentStorageAgreement { agent: Box<dyn IsAgent>, h_bank: Box<dyn IsHBank> },
    AgentExchangeAgreement { agent: Box<dyn IsAgent>, h_bank: Box<dyn IsHBank> },
    GeneratorStorageAgreement { generator: Box<dyn IsGenerator>, h_bank: Box<dyn IsHBank> },
    GeneratorExchangeAgreement { generator: Box<dyn IsGenerator>, h_bank: Box<dyn IsHBank> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum DonationLegalStructure {
    PhilanthropicAgreement { donor: Box<dyn IsDonor>, h_bank: Box<dyn IsHBank> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AdLegalStructure {
    AdvertiserAgreement { advertiser: Box<dyn IsAdvertiser>, h_bank: Box<dyn IsHBank> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionLegalStructure {
    ConsultAgreement { agent: Box<dyn IsAgent>, consultant: Box<dyn IsConsultant>, h_bank: Box<dyn IsHBank> },
    DirectSale { agent_a: Box<dyn IsAgent>, agent_b: Box<dyn IsAgent>, generators: Vec<Box<dyn IsGenerator>>, h_bank: Box<dyn IsHBank> },
    PurchaseAgreement { agent_a: Box<dyn IsAgent>, agent_b: Box<dyn IsAgent>, generators: Vec<Box<dyn IsGenerator>>, h_bank: Box<dyn IsHBank> },
    LicensingAgreement { agent_a: Box<dyn IsAgent>, agent_b: Box<dyn IsAgent>, generators: Vec<Box<dyn IsGenerator>>, h_bank: Box<dyn IsHBank> },
    AccessAgreement { agent_a: Box<dyn IsAgent>, agent_b: Box<dyn IsAgent>, generators: Vec<Box<dyn IsGenerator>>, h_bank: Box<dyn IsHBank> },
    SubscriptionAgreement { agent_a: Box<dyn IsAgent>, agent_b: Box<dyn IsAgent>, generators: Vec<Box<dyn IsGenerator>>, h_bank: Box<dyn IsHBank> },
    ConsortiumAgreement { agents: Vec<Box<dyn IsAgent>>, generators: Vec<Box<dyn IsGenerator>>, h_bank: Box<dyn IsHBank> },
    ParticipationAgreement { agents: Vec<Box<dyn IsAgent>>, funders: Vec<Box<dyn IsFunder>>, generators: Vec<Box<dyn IsGenerator>>, h_bank: Box<dyn IsHBank> },
    DataExchangeAgreement { agents_a: Vec<Box<dyn IsAgent>>, agents_b: Vec<Box<dyn IsAgent>>, generators: Vec<Box<dyn IsGenerator>>, h_bank: Box<dyn IsHBank> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TwoPartyLegalStructure {
    Storage_or_Exchange(StorageExchangeLegalStructure),
    Donation(DonationLegalStructure),
    Advertisement(AdLegalStructure),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContractCategory {
    TwoParty(TwoPartyLegalStructure),
    ThreePlusParty(TransactionLegalStructure),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContractLegalFramework {
    UCC,
    CommonLaw,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GeneratorRateSpecification {
    KnowledgeRate(f64),
    UsageRate(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IndividualContributionLevel {
    DataOnly,
    DataAndParticipation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataPrivacyLevel {
    HIPPA_minus,
    HIPPA_deidentified,
}

#[derive(Debug, Clone, Default)]
pub struct Terms {
    pub data_borrowers_full_list: Option<Vec<String>>,
    pub data_request_explanation: Option<String>,
    pub data_request_purpose_executive_summary: Option<String>,
}