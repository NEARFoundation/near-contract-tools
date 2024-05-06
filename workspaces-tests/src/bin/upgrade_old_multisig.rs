workspaces_tests::predicate!();

use near_sdk::{env, json_types::Base64VecU8, near, BorshStorageKey, PanicOnDefault};
use near_sdk_contract_tools::{
    approval::{self, ApprovalManager},
    owner::*,
    rbac::Rbac,
    Owner, Rbac, SimpleMultisig, Upgrade,
};

#[derive(BorshStorageKey, Debug, Clone)]
#[near]
pub enum Role {
    Multisig,
}

#[derive(Debug, Clone)]
#[near(serializers = [borsh, json])]
pub enum ContractAction {
    Upgrade { code: Base64VecU8 },
}

impl approval::Action<Contract> for ContractAction {
    type Output = ();

    fn execute(self, _contract: &mut Contract) -> Self::Output {
        match self {
            ContractAction::Upgrade { code } => _contract.upgrade(code.into()),
        }
    }
}

#[derive(Owner, Debug, Clone, Rbac, Upgrade, SimpleMultisig, PanicOnDefault)]
#[rbac(roles = "Role")]
#[simple_multisig(role = "Role::Multisig", action = "ContractAction")]
#[upgrade(serializer = "borsh", hook = "owner")]
#[near(contract_state)]
pub struct Contract {
    pub foo: u32,
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let mut contract = Self { foo: 0 };

        ApprovalManager::init(
            &mut contract,
            approval::simple_multisig::Configuration::new(1, 0),
        );

        let predecessor = env::predecessor_account_id();

        Owner::init(&mut contract, &predecessor);

        contract.add_role(&predecessor, &Role::Multisig);

        contract
    }

    pub fn request(&mut self, request: ContractAction) -> u32 {
        self.create_request(request, Default::default()).unwrap()
    }

    pub fn approve(&mut self, request_id: u32) {
        self.approve_request(request_id).unwrap()
    }

    pub fn execute(&mut self, request_id: u32) {
        env::log_str("executing request");
        self.execute_request(request_id).unwrap()
    }
}
