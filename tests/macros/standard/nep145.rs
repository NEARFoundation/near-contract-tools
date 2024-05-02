use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, json_types::U128, log, near_bindgen, store::LookupMap, AccountId, PanicOnDefault,
};
use near_sdk_contract_tools::{hook::Hook, standard::nep145::*, Nep145};

#[derive(BorshSerialize, BorshDeserialize)]
#[borsh(crate = "near_sdk::borsh")]
#[derive(PanicOnDefault, Nep145)]
#[nep145(force_unregister_hook = "ForceUnregisterHook")]
#[near_bindgen]
pub struct Contract {
    pub storage: LookupMap<AccountId, Vec<u64>>,
}

pub struct ForceUnregisterHook;

impl Hook<Contract, Nep145ForceUnregister<'_>> for ForceUnregisterHook {
    fn hook<R>(
        contract: &mut Contract,
        _args: &Nep145ForceUnregister<'_>,
        f: impl FnOnce(&mut Contract) -> R,
    ) -> R {
        log!("Before force unregister");
        let r = f(contract);
        log!("After force unregister");
        r
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let mut contract = Self {
            storage: LookupMap::new(b"s"),
        };

        Nep145Controller::set_storage_balance_bounds(
            &mut contract,
            &StorageBalanceBounds {
                min: U128(0),
                max: None,
            },
        );

        contract
    }

    pub fn use_storage(&mut self, num: u64) {
        let storage_usage_start = env::storage_usage();

        let predecessor = env::predecessor_account_id();

        self.storage.insert(predecessor.clone(), (0..num).collect());

        self.storage.flush();

        let storage_usage = env::storage_usage() - storage_usage_start;
        let storage_fee = env::storage_byte_cost().saturating_mul(u128::from(storage_usage));

        Nep145Controller::lock_storage(self, &predecessor, storage_fee.as_yoctonear().into())
            .unwrap_or_else(|e| env::panic_str(&format!("Storage lock error: {}", e)));
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env, NearToken};

    use super::*;

    fn alice() -> AccountId {
        "alice.near".parse().unwrap()
    }

    #[test]
    fn storage_sanity_check() {
        let one_near = NearToken::from_near(1u128);
        let byte_cost = env::storage_byte_cost();

        let mut contract = Contract::new();

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(alice())
            .attached_deposit(one_near)
            .build());

        Nep145::storage_deposit(&mut contract, None, None);

        assert_eq!(
            Nep145::storage_balance_of(&contract, alice()),
            Some(StorageBalance {
                total: U128(one_near.as_yoctonear()),
                available: U128(one_near.as_yoctonear()),
            }),
        );

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(alice())
            .build());

        contract.use_storage(1000);

        let first = Nep145::storage_balance_of(&contract, alice()).unwrap();

        assert_eq!(first.total.0, one_near.as_yoctonear());
        assert!(
            one_near.as_yoctonear() - (first.available.0 + 8 * 1000 * byte_cost.as_yoctonear())
                < 100 * byte_cost.as_yoctonear()
        ); // about 100 bytes for storing keys, etc.

        contract.use_storage(2000);

        let second = Nep145::storage_balance_of(&contract, alice()).unwrap();

        assert_eq!(second.total.0, one_near.as_yoctonear());
        assert_eq!(
            second.available.0,
            first.available.0 - 8 * 1000 * byte_cost.as_yoctonear()
        );
    }
}
