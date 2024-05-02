#![allow(missing_docs)]

workspaces_tests::predicate!();

use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    env,
    near_bindgen,
    store::Vector,
    NearToken, PanicOnDefault, Promise,
};
use near_sdk_contract_tools::utils::apply_storage_fee_and_refund;

#[derive(BorshSerialize, BorshDeserialize)]
#[borsh(crate = "near_sdk::borsh")]
#[derive(PanicOnDefault)]
#[near_bindgen]
pub struct ContractBad {
    pub items: Vector<String>,
}

#[near_bindgen]
impl ContractBad {
    #[init]
    pub fn new() -> Self {
        Self {
            items: Vector::new(b"i"),
        }
    }

    pub fn storage_byte_cost(&self) -> NearToken {
        env::storage_byte_cost()
    }

    #[payable]
    pub fn store(&mut self, item: String) -> Option<Promise> {
        let initial_storage_usage = env::storage_usage();

        self.items.push(item);

        self.items.flush(); // Force write before sending refund

        apply_storage_fee_and_refund(initial_storage_usage, 0)
    }
}
