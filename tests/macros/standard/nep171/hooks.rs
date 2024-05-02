use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, PanicOnDefault};
use near_sdk_contract_tools::{hook::Hook, nft::*};

#[derive(BorshSerialize, BorshDeserialize)]
#[borsh(crate = "near_sdk::borsh")]
#[derive(PanicOnDefault, Nep171)]
#[nep171(transfer_hook = "Self")]
#[near_bindgen]
pub struct Contract {
    transfer_count: u32,
}

impl Hook<Contract, Nep171Transfer<'_>> for Contract {
    fn hook<R>(
        contract: &mut Contract,
        args: &Nep171Transfer<'_>,
        f: impl FnOnce(&mut Contract) -> R,
    ) -> R {
        log!(
            "{:?} is transferring {} to {}",
            args.sender_id,
            args.token_id,
            args.receiver_id,
        );
        let r = f(contract);
        contract.transfer_count += 1;
        r
    }
}
