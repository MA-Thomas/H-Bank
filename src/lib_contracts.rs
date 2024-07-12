use lib_structs_enums::{IsHBank, IsAgent, IsFunder, IsDonor, Party,
    HBank, DataOriginator, DataCustodian, DataRecipient, Funder, Donor,
    StorageLegalStructure, DonationLegalStructure, TransactionLegalStructure,
    TwoPartyLegalStructure, ContractCategory,
    ContractLegalFramework};

// ********* BRING IT ALL TOGETHER. USE THE DATA TYPES ABOVE TO DEFINE A HEALTHDATACONTRACT ********* 
pub struct HealthDataContract<A: IsAgent, B: IsAgent, D: IsDonor, F: IsFunder, H: IsHBank> {
    /*
     This definition of HealthDataContract does not imply that all of the generic parameters (A, B, D, F, H) are required 
     for each instantiated instance of HealthDataContract. In Rust, generics allow you to specify types that can vary 
     across different instances of a struct or function. 
     When you define a struct with generics (<A: IsAgent, B: IsAgent, D: IsDonor, F: IsFunder, H: IsHBank>), you're specifying 
     that HealthDataContract can be instantiated with any combination of types that satisfy these constraints.
     You can instantiate HealthDataContract with only the necessary types that are relevant for a specific contract. 
     For instance, if your contract involves only an A (agent) and an H (HBank), you can instantiate it with just those types.
     */
    parties: Vec<Box<dyn Party>>,
    agreement_type: ContractCategory<A, B, D, F, H>,
    legal_framework: ContractLegalFramework,
    terms: String,
}

impl<A: IsAgent, B: IsAgent, D: IsDonor, F: IsFunder, H: IsHBank> HealthDataContract<A, B, D, F, H> {
    pub fn new(parties: Vec<Box<dyn Party>>, agreement_type: ContractCategory<A, B, D, F, H>, legal_framework: ContractLegalFramework, terms: String) -> Self {
        HealthDataContract {
            parties,
            agreement_type,
            legal_framework,
            terms,
        }
    }

    pub fn add_party(&mut self, party: Box<dyn Party>) {
        self.parties.push(party);
    }
    pub fn add_terms(&mut self, terms: &str) {
        self.terms.push_str(terms);
    }

    // Other methods as needed
}
