pub enum Agent {
    DataOriginator, // The person from whom the data was collected.
    DataCustodian,  // The person or entity that has effective control over data when there is no applicable DataOriginator.
    DataRecipient,  // The person or entity receiving access to or effective control of data.
    Funder,         // The person or entity providing funding (if different from the DataRecipient, or if there is no DataRecipient).
    HBank,          // The health-data bank. AKA, "We", "Us", etc.
}

pub enum ContractLegalFramework {
    UCC,        // For tangible goods, e.g., hardware, biological samples, etc.
    CommonLaw,  // For services, intellectual property. THIS IS THE DEFAULT FOR H-BANK.
}

pub enum ContractCategory {
    TwoParty(StorageLegalStructure),
    ThreeParty(TransactionLegalStructure),
}

pub enum TransactionLegalStructure {
    // *** These are 3-party contracts ***

    LicensingAgreement,     // The data recipient is granted specific rights to use the data under defined conditions, which can include the ability to modify, reproduce, distribute, and create derivative works based on the data. The license can be for a specific period or perpetual (but revokable). The agreement often specifies the allowed and prohibited uses of the data, which can include commercial exploitation. The recipient typically pays a fee, which can be a one-time payment or recurring. There might also be royalties based on the revenue generated from using the data.
    AccessAgreement,        // A more limited version of the LicensingAgreement. The recipient can view and use the data but often without the broader rights to modify, reproduce, distribute or commercialize the data.
    SubscriptionAgreement,  // Regular payments are made by the data recipient to continuously access updated health data. Involves recurring payments for ongoing access to potentially new or changing health data, often with provisions for updates and support. There may be subscription tiers, pricing structures, and access privileges.

    ResearchCollaboration,  // The data originator collaborates with the data recipient (typically a research institution) in a joint research project where access to health data is granted in exchange for funding or resources. Specifies the research objectives, funding arrangements, data sharing protocols, and intellectual property rights related to research outcomes.
    GrantFunding,           // Research institutions or funding bodies provide grants to the data originator in exchange for access to health data for research purposes. Grants may fund specific research projects, with terms related to data access, use, publication rights, and compliance with regulatory requirements.
    DataExchangeAgreement,  // An agreement between two data agents where health data is exchanged. Defines the terms of data exchange, including data formats, protocols for data transmission, security measures, any reciprocal benefits and compensation.

    // These require strict ownership to be established. USE WITH CAUTION.
    DirectSale,        // The originator sells their health data to a recipient for a one-time or recurring payments. Ownership of health data is transferred outright to the recipient, typically for a lump-sum payment or installment payments.
    PurchaseAgreement, // A contractual agreement where ownership of health data is transferred from one agent to another agent in exchange for compensation. Specifies terms of sale, including price, transfer of ownership, and any warranties or liabilities related to the data.
}

pub enum StorageLegalStructure {
    // *** These are 2-party contracts ***

    DataStorageAgreement, // A non H-Bank agent contributes health data to HBank for storage, management, and potential commercialization. Specifies transfer mechanisms, usage rights granted to HBank, data security requirements, compensation terms (if applicable), warranties, indemnification provisions, and termination clauses.
}
