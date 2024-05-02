#![allow(missing_docs)]

workspaces_tests::predicate!();

use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
#[borsh(crate = "near_sdk::borsh")]
#[near_bindgen]
pub struct ContractBad {
    pub foo: u32,
}

#[near_bindgen]
impl ContractBad {
    #[init]
    pub fn new() -> Self {
        Self { foo: 0 }
    }

    pub fn increment_foo(&mut self) {
        self.foo += 1;
    }

    pub fn get_foo(&self) -> u32 {
        self.foo
    }
}
