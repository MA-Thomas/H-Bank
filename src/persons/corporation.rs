use std::collections::HashSet;
use regex::Regex;

use time::Date;


/*
An Corporation is linked uniquely to their person_id. 
For privacy reasons, only the fields: person_id and data_of_birth are public as they are used by health_data_contract->validate_age_wrt_agency_privacy().

*/
#[derive(Debug)]
pub struct Corporation {
    pub name: String,
    pub person_id: String,
    pub tax_id: Option<String>,

}


impl Corporation {
    pub fn new(name: String, person_id: String) -> Self {
        Corporation {
            name,
            person_id,
            tax_id: None,
        }
    }

    pub fn get_person_id(&self) -> &str {
        &self.person_id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }

    // Function to validate EIN format (e.g., XX-XXXXXXX)
    pub fn validate_ein_format(ein: &str) -> bool {
        // Check if the EIN matches the pattern of two digits, a dash, followed by seven digits
        let ein_pattern = regex::Regex::new(r"^\d{2}-\d{7}$").unwrap();
        ein_pattern.is_match(ein)
    }
    // Function to add a tax ID (EIN) to the individual if it is valid
    pub fn add_tax_id(&mut self, ein: &str) {
        if Self::validate_ein_format(ein) {
            self.tax_id = Some(ein.to_string()); // Set the tax_id if EIN is valid
        } else {
            eprintln!("Invalid EIN format detected.");
        }
    }

}

