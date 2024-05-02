#![allow(missing_docs)]

workspaces_tests::predicate!();

use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
#[borsh(crate = "near_sdk::borsh")]
#[near_bindgen]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_five(&self, value: u32) -> u32 {
        value + 5
    }
}
