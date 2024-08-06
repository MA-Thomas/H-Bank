use std::collections::HashSet;
use regex::Regex;

use time::Date;
use crate::contracts::structs_enums::EntityId;

/*
An Individual is linked uniquely to their person_id. 
A Corporation is also linked uniquely to their person_id.
For privacy reasons, only the fields: person_id and data_of_birth are public as they are used by health_data_contract->validate_age_wrt_agency_privacy().

*/
#[derive(Debug)]
pub struct Individual {
    pub name: String,
    pub person_id: EntityId,
    pub hla_profile: Option<String>,
    pub blood_type: Option<String>,
    pub date_of_birth: Date,
}


impl Individual {
    pub fn new(name: String, person_id: EntityId, date_of_birth: Date) -> Self {
        Individual {
            name,
            person_id,
            hla_profile: None,
            blood_type: None,
            date_of_birth,
        }
    }

    pub fn get_person_id(&self) -> &EntityId {
        &self.person_id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_hla_profile(&mut self, alleles: Vec<&str>) {
        if Self::validate_hla_alleles(&alleles) {
            self.hla_profile = Some(Self::create_sorted_hla_profile(&alleles));
        } else {
            eprintln!("Invalid HLA allele format detected.");
        }
    }

    pub fn add_blood_type(&mut self, blood_type: &str) {
        self.blood_type = Some(blood_type.to_string());
    }

    pub fn create_sorted_hla_profile(alleles: &[&str]) -> String {
        let mut sorted_alleles: Vec<String> = alleles.iter().map(|&s| s.to_string()).collect();
        sorted_alleles.sort(); // Sort alphabetically
        sorted_alleles.join(",") // Join with a comma
    }

    // Validate HLA alleles to ensure correct format (e.g., "A02:01")
    pub fn validate_hla_alleles(alleles: &[&str]) -> bool {
        let hla_pattern = Regex::new(r"^[A-Z][0-9]{2}:[0-9]{2}$").unwrap();
        alleles.iter().all(|&allele| hla_pattern.is_match(allele))
    }
}


