use near_sdk::{env, near, PanicOnDefault};
use near_sdk_contract_tools::{
    nft::*,
    owner::Owner,
    pause::{hooks::PausableHook, Pause},
    Owner, Pause,
};

#[derive(NonFungibleToken, Pause, Owner, PanicOnDefault)]
#[non_fungible_token(transfer_hook = "PausableHook")]
#[near(contract_state)]
pub struct Contract {
    next_token_id: u32,
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let mut contract = Self { next_token_id: 0 };

        contract.set_contract_metadata(&ContractMetadata::new(
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
        self.mint_with_metadata(
            token_id.clone(),
            env::predecessor_account_id(),
            &TokenMetadata::new()
                .title(format!("Token {token_id}"))
                .description(format!("This is token {token_id}.")),
        )
        .unwrap_or_else(|e| env::panic_str(&format!("Minting failed: {e}")));

        token_id
    }
}
