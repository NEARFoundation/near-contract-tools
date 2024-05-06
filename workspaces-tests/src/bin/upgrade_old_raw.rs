workspaces_tests::predicate!();

use near_sdk::{env, near, require, PanicOnDefault};
use near_sdk_contract_tools::{owner::*, upgrade::PostUpgrade, Owner};

#[derive(Owner, PanicOnDefault)]
#[near(contract_state)]
pub struct ContractOld {
    pub foo: u32,
}

#[near]
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
    env::setup_panic_hook();

    let predecessor = env::predecessor_account_id();
    let owner = ContractOld::slot_owner().read();
    require!(Some(predecessor) == owner, "Owner only");

    unsafe {
        near_sdk_contract_tools::upgrade::raw::upgrade(PostUpgrade::default());
    }
}
