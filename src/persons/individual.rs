use std::collections::HashSet;
use regex::Regex;

use time::Date;


/*
An Individual is linked uniquely to their person_id. 
A Corporation is also linked uniquely to their person_id.
For privacy reasons, only the fields: person_id and data_of_birth are public as they are used by health_data_contract->validate_age_wrt_agency_privacy().

*/
#[derive(Debug)]
pub struct Individual {
    pub name: String,
    pub person_id: String,
    pub hla_profile: Option<String>,
    pub blood_type: Option<String>,
    pub date_of_birth: Date,
}


impl Individual {
    pub fn new(name: String, person_id: String, date_of_birth: Date) -> Self {
        Individual {
            name,
            person_id,
            hla_profile: None,
            blood_type: None,
            date_of_birth,
        }
    }

    pub fn get_person_id(&self) -> &str {
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

// pub fn main() {
//     let mut person = Individual::new("Alice".to_string(), "1");

//     // Example valid alleles
//     let valid_alleles = vec!["A02:01", "B07:02", "C07:02"];
//     person.add_hla_profile(valid_alleles);

//     // Example invalid alleles
//     let invalid_alleles = vec!["A0201", "B07:02"];
//     person.add_hla_profile(invalid_alleles);

//     // Example blood type
//     person.add_blood_type("O+");

//     println!("{:?}", person);
// }
