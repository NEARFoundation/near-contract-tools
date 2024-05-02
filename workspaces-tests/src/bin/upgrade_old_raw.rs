#![allow(missing_docs)]

workspaces_tests::predicate!();

use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    env, near_bindgen, PanicOnDefault,
};
use near_sdk_contract_tools::{owner::*, upgrade::PostUpgrade, Owner};

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault, Owner)]
#[borsh(crate = "near_sdk::borsh")]
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

#[no_mangle]
pub fn upgrade() {
    near_sdk::env::setup_panic_hook();

    ContractOld::require_owner();

    unsafe {
        near_sdk_contract_tools::upgrade::raw::upgrade(PostUpgrade::default());
    }
}
