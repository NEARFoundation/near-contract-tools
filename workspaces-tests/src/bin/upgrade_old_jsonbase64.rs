#![allow(missing_docs)]

workspaces_tests::predicate!();

use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    env, near_bindgen, PanicOnDefault,
};
use near_sdk_contract_tools::{owner::*, Owner, Upgrade};

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault, Owner, Upgrade)]
#[borsh(crate = "near_sdk::borsh")]
#[upgrade(serializer = "jsonbase64", hook = "owner")]
#[near_bindgen]
pub struct ContractOld {
    pub foo: u32,
}

#[near_bindgen]
impl ContractOld {
    #[init]
    pub fn new() -> Self {
        let mut contract = Self { foo: 0 };

        Owner::init(&mut contract, &env::predecessor_account_id());
        contract
    }

    pub fn increment_foo(&mut self) {
        self.foo += 1;
    }

    pub fn get_foo(&self) -> u32 {
        self.foo
    }
}
