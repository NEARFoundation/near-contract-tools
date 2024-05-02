workspaces_tests::predicate!();

use near_sdk::{near, PanicOnDefault};

#[derive(PanicOnDefault)]
#[near(contract_state)]
pub struct Contract {}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_five(&self, value: u32) -> u32 {
        value + 5
    }
}
