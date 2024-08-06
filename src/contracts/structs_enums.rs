use std::fmt::Debug;
use time::Date;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityId(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct PartyInfo {
    pub name: String,
    pub entity_id: EntityId,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Party {
    HBank(PartyInfo),
    DataOriginator(PartyInfo),
    DataCustodian(PartyInfo),
    DataRecipient(PartyInfo),
    DataConsultant(PartyInfo),
    DataGenerator(PartyInfo),
    Funder(PartyInfo),
    Donor(PartyInfo),
    Advertiser(PartyInfo),
}

impl Party {
    fn new(name: String, entity_id: EntityId, party_type: PartyType) -> Self {
        let info = PartyInfo { name, entity_id };
        match party_type {
            PartyType::HBank => Party::HBank(info),
            PartyType::DataOriginator => Party::DataOriginator(info),
            PartyType::DataCustodian => Party::DataCustodian(info),
            PartyType::DataRecipient => Party::DataRecipient(info),
            PartyType::DataConsultant => Party::DataConsultant(info),
            PartyType::DataGenerator => Party::DataGenerator(info),
            PartyType::Funder => Party::Funder(info),
            PartyType::Donor => Party::Donor(info),
            PartyType::Advertiser => Party::Advertiser(info),
        }
    }

    fn as_party_info(&self) -> &PartyInfo {
        match self {
            Party::HBank(info) => info,
            Party::DataOriginator(info) => info,
            Party::DataCustodian(info) => info,
            Party::DataRecipient(info) => info,
            Party::DataConsultant(info) => info,
            Party::DataGenerator(info) => info,
            Party::Funder(info) => info,
            Party::Donor(info) => info,
            Party::Advertiser(info) => info,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PartyType {
    HBank,
    DataOriginator,
    DataCustodian,
    DataRecipient,
    DataConsultant,
    DataGenerator,
    Funder,
    Donor,
    Advertiser,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StorageExchangeLegalStructure {
    AgentStorageAgreement { agent: Party, h_bank: Party },
    AgentExchangeAgreement { agent: Party, h_bank: Party },
    GeneratorStorageAgreement { generator: Party, h_bank: Party },
    GeneratorExchangeAgreement { generator: Party, h_bank: Party },
}

#[derive(Debug, Clone, PartialEq)]
pub enum DonationLegalStructure {
    PhilanthropicAgreement { donor: Party, h_bank: Party },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AdLegalStructure {
    AdvertiserAgreement { advertiser: Party, h_bank: Party },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionLegalStructure {
    ConsultAgreement { agent: Party, consultant: Party, h_bank: Party },
    DirectSale { agent_a: Party, agent_b: Party, generators: Vec<Party>, h_bank: Party },
    PurchaseAgreement { agent_a: Party, agent_b: Party, generators: Vec<Party>, h_bank: Party },
    LicensingAgreement { agent_a: Party, agent_b: Party, generators: Vec<Party>, h_bank: Party },
    AccessAgreement { agent_a: Party, agent_b: Party, generators: Vec<Party>, h_bank: Party },
    SubscriptionAgreement { agent_a: Party, agent_b: Party, generators: Vec<Party>, h_bank: Party },
    ConsortiumAgreement { agents: Vec<Party>, generators: Vec<Party>, h_bank: Party },
    ParticipationAgreement { agents: Vec<Party>, funders: Vec<Party>, generators: Vec<Party>, h_bank: Party },
    DataExchangeAgreement { agents_a: Vec<Party>, agents_b: Vec<Party>, generators: Vec<Party>, h_bank: Party },
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

