workspaces_tests::predicate!();

use near_sdk::{env, near, AccountId, BorshStorageKey, PanicOnDefault, Promise};
use near_sdk_contract_tools::{
    approval::{
        native_transaction_action::{self, NativeTransactionAction},
        simple_multisig::{ApprovalState, Configuration},
        ApprovalManager,
    },
    rbac::Rbac,
    Rbac, SimpleMultisig,
};

#[derive(BorshStorageKey, Clone, Debug)]
#[near]
pub enum Role {
    Multisig,
}

#[derive(Rbac, SimpleMultisig, PanicOnDefault)]
#[simple_multisig(action = "NativeTransactionAction", role = "Role::Multisig")]
#[rbac(roles = "Role")]
#[near(contract_state)]
pub struct Contract {}

#[near]
impl Contract {
    const APPROVAL_THRESHOLD: u8 = 2;
    const VALIDITY_PERIOD: u64 = 1_000_000 * 1_000 * 60 * 60 * 24 * 7;

    #[init]
    pub fn new() -> Self {
        <Self as ApprovalManager<_, _, _>>::init(Configuration::new(
            Self::APPROVAL_THRESHOLD,
            Self::VALIDITY_PERIOD,
        ));

        Self {}
    }

    pub fn obtain_multisig_permission(&mut self) {
        self.add_role(&env::predecessor_account_id(), &Role::Multisig);
    }

    pub fn request(
        &mut self,
        receiver_id: AccountId,
        actions: Vec<native_transaction_action::PromiseAction>,
    ) -> u32 {
        let request_id = self
            .create_request(
                native_transaction_action::NativeTransactionAction {
                    receiver_id,
                    actions,
                },
                ApprovalState::new(),
            )
            .unwrap();

        near_sdk::log!(format!("Request ID: {request_id}"));

        request_id
    }

    pub fn approve(&mut self, request_id: u32) {
        self.approve_request(request_id).unwrap();
    }

    pub fn is_approved(&self, request_id: u32) -> bool {
        <Contract as ApprovalManager<_, _, _>>::is_approved_for_execution(request_id).is_ok()
    }

    pub fn execute(&mut self, request_id: u32) -> Promise {
        self.execute_request(request_id).unwrap()
    }

    #[private]
    pub fn private_add_one(&mut self, value: u32) -> u32 {
        value + 1
    }
}
