use std::collections::HashSet;
use regex::Regex;


/*
A Person is linked uniquely to their person_id. For privacy reasons, Persons will not appear 
in contracts since contracts are made public.
DataOriginator's appear in contracts and their entity_id derives from (but is not identical to) 
the person_id of the corresponding person.

Idea: entity_id = hash(person_id, date and time of account opening, previous hash)
Entity_id's are updated randomly and multiple times per year. HBank maintains a list of entity_ids for each Person.


TODO: Translate this into the code.
*/
#[derive(Debug)]
struct Person {
    name: String,
    person_id: String,
    hla_profile: Option<String>,
    blood_type: Option<String>,
}

impl Person {
    pub fn new(name: String, person_id: u64) -> Self {
        Person {
            name,
            person_id,
            hla_profile: None,
            blood_type: None,
        }
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

pub fn main() {
    let mut person = Person::new("Alice".to_string(), 1);

    // Example valid alleles
    let valid_alleles = vec!["A02:01", "B07:02", "C07:02"];
    person.add_hla_profile(valid_alleles);

    // Example invalid alleles
    let invalid_alleles = vec!["A0201", "B07:02"];
    person.add_hla_profile(invalid_alleles);

    // Example blood type
    person.add_blood_type("O+");

    println!("{:?}", person);
}
