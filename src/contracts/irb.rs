/*
Declare the submodules within the irb module.
*/


/*
Notes:

The contract should have the ability to query an IRB approvals database using the contract_id as key 
- during contract validation.
If the approval is found, the irb_approved field is set to true (before the irb validation method)

This IRB approvals database is filled as IRBs submit approvals via our API.
*/
pub mod irb_api;
