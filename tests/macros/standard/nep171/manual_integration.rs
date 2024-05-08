use near_sdk::{env, near, PanicOnDefault};
use near_sdk_contract_tools::{
    hook::Hook,
    owner::Owner,
    pause::Pause,
    standard::{
        nep171::*,
        nep177::{self, Nep177Controller},
        nep178, nep181,
    },
    Nep171, Nep177, Nep178, Nep181, Owner, Pause,
};

#[derive(Nep171, Nep177, Nep178, Nep181, Pause, Owner, PanicOnDefault)]
#[nep171(
    all_hooks = "(nep178::TokenApprovals, nep181::TokenEnumeration)",
    transfer_hook = "Self",
    check_external_transfer = "nep178::TokenApprovals",
    token_data = "(nep177::TokenMetadata, nep178::TokenApprovals)"
)]
#[nep178()]
#[near(contract_state)]
pub struct Contract {
    next_token_id: u32,
}

impl Hook<Contract, action::Nep171Transfer<'_>> for Contract {
    fn hook<R>(
        contract: &mut Contract,
        _args: &action::Nep171Transfer<'_>,
        f: impl FnOnce(&mut Contract) -> R,
    ) -> R {
        Contract::require_unpaused();
        f(contract)
    }
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let mut contract = Self { next_token_id: 0 };

        contract.set_contract_metadata(&nep177::ContractMetadata::new(
            "My NFT".to_string(),
            "MYNFT".to_string(),
            None,
        ));

        Owner::init(&mut contract, &env::predecessor_account_id());

        contract
    }

    pub fn mint(&mut self) -> TokenId {
        Self::require_unpaused();

        let token_id = format!("token_{}", self.next_token_id);
        self.next_token_id += 1;
        Nep177Controller::mint_with_metadata(
            self,
            &token_id,
            &env::predecessor_account_id(),
            &nep177::TokenMetadata::new()
                .title(format!("Token {token_id}"))
                .description(format!("This is token {token_id}.")),
        )
        .unwrap_or_else(|e| env::panic_str(&format!("Minting failed: {e}")));

        token_id
    }
}
