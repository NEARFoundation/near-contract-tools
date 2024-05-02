use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    env, near_bindgen,
};
use near_sdk_contract_tools::{migrate::MigrateHook, Migrate};

mod old {
    use super::*;

    #[derive(BorshSerialize, BorshDeserialize, Debug)]
    #[borsh(crate = "near_sdk::borsh")]
    #[near_bindgen]
    pub struct Old {
        pub foo: u64,
    }

    #[near_bindgen]
    impl Old {
        #[init]
        pub fn new(foo: u64) -> Self {
            Self { foo }
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Migrate)]
#[borsh(crate = "near_sdk::borsh")]
#[migrate(from = "old::Old")]
#[near_bindgen]
struct MyContract {
    pub bar: u64,
}

impl MigrateHook for MyContract {
    fn on_migrate(old: old::Old) -> Self {
        Self { bar: old.foo }
    }
}

#[test]
fn default_from() {
    let old = old::Old::new(99);

    // This is done automatically in real #[near_bindgen] WASM contracts
    env::state_write(&old);

    assert_eq!(old.foo, 99);

    let migrated = MyContract::migrate();

    assert_eq!(migrated.bar, 99);
}
