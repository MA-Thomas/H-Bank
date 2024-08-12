/*
Define the top level modules

Software Architecture Design Philosophy: Modular Monolith (perhaps migrating to Microservices as scale/problems demand in the future)
*/
pub mod contracts;
pub mod persons;
pub mod api;

// re-export public-API modules from here, the top level. This way, other crates can call this
// as: use h_bank::public_api;
pub use api::public_api as public_api;

