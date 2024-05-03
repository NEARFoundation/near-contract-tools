workspaces_tests::predicate!();

use near_sdk::{env, log, near, PanicOnDefault};
use near_sdk_contract_tools::{hook::Hook, standard::nep171::*, Nep171};

#[derive(Nep171, PanicOnDefault)]
#[nep171(transfer_hook = "Self")]
#[near(contract_state)]
pub struct Contract {}

impl Hook<Contract, action::Nep171Transfer<'_>> for Contract {
    fn hook<R>(
        contract: &mut Contract,
        args: &action::Nep171Transfer<'_>,
        f: impl FnOnce(&mut Contract) -> R,
    ) -> R {
        log!("before_nft_transfer({})", args.token_id);
        let r = f(contract);
        log!("after_nft_transfer({})", args.token_id);
        r
    }
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {}
    }

    pub fn mint(&mut self, token_ids: Vec<TokenId>) {
        let action = action::Nep171Mint {
            token_ids: &token_ids,
            receiver_id: &env::predecessor_account_id(),
            memo: None,
        };
        Nep171Controller::mint(self, &action)
            .unwrap_or_else(|e| env::panic_str(&format!("Failed to mint: {:#?}", e)));
    }
}
