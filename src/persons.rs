/*
Declare the submodules within the module: persons.
*/

pub mod individual;
pub mod corporation;

/*
You can re-export an entire submodule in Rust using pub use. 
This allows you to expose the entire module at a higher level, making it easier for users 
of your crate to access all items within that submodule through a single import.
*/
pub use individual::*;  // Re-export all public items from health_data_contract
pub use corporation::*;  // Re-export all public items from structs_enums
