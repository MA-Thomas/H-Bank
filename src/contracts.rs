/*
Declare the submodules within the contracts module.
*/

pub mod health_data_contract;
pub mod structs_enums;
pub mod cohorts;
pub mod irb;


/*
You can re-export an entire submodule in Rust using pub use. 
This allows you to expose the entire module at a higher level, making it easier for users 
of your crate to access all items within that submodule through a single import.
*/
pub use health_data_contract::*;  // Re-export all public items from health_data_contract
pub use structs_enums::*;  // Re-export all public items from structs_enums