workspaces_tests::predicate!();

use near_sdk::{env, log, near, AccountId, NearToken, PanicOnDefault, PromiseOrValue};
use near_sdk_contract_tools::standard::nep171::*;

#[derive(PanicOnDefault)]
#[near(contract_state)]
pub struct Contract {}

#[near]
impl Nep171Receiver for Contract {
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> PromiseOrValue<bool> {
        log!(
            "Received {} from {} via {}",
            token_id,
            previous_owner_id,
            sender_id,
        );

        if msg == "panic" {
            near_sdk::env::panic_str("panic requested");
        } else if let Some(account_id) = msg.strip_prefix("transfer:") {
            log!("Transferring {} to {}", token_id, account_id);
            return ext_nep171::ext(env::predecessor_account_id())
                .with_attached_deposit(NearToken::from_yoctonear(1u128))
                .nft_transfer(account_id.parse().unwrap(), token_id, None, None)
                .then(Contract::ext(env::current_account_id()).return_true()) // ask to return the token even though we don't own it anymore
                .into();
        }

        PromiseOrValue::Value(msg == "return")
    }
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {}
    }

    pub fn return_true(&self) -> bool {
        log!("returning true");
        true
    }
}
