/*
Define the top level modules

Software Architecture Design Philosophy: Modular Monolith (perhaps migrating to Microservices as scale/problems demand in the future)
*/
pub mod contracts;
pub mod persons;

// re-export public-API modules from here, the top level. This way, other crates can call this
// as: use h_bank::cohort_api;
pub use contracts::cohorts::public_api as cohort_api;

