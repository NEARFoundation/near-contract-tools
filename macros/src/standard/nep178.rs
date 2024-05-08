use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, Type};

use crate::unitify;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(nep178), supports(struct_named))]
pub struct Nep178Meta {
    pub storage_key: Option<Expr>,
    pub all_hooks: Option<Type>,
    pub approve_hook: Option<Type>,
    pub revoke_hook: Option<Type>,
    pub revoke_all_hook: Option<Type>,

    pub generics: syn::Generics,
    pub ident: syn::Ident,

    // crates
    #[darling(rename = "crate", default = "crate::default_crate_name")]
    pub me: syn::Path,
    #[darling(default = "crate::default_near_sdk")]
    pub near_sdk: syn::Path,
}

pub fn expand(meta: Nep178Meta) -> Result<TokenStream, darling::Error> {
    let Nep178Meta {
        storage_key,
        all_hooks,
        approve_hook,
        revoke_hook,
        revoke_all_hook,

        generics,
        ident,

        me,
        near_sdk,
    } = meta;

    let (imp, ty, wher) = generics.split_for_impl();

    let root = storage_key.map(|storage_key| {
        quote! {
            fn root() -> #me::slot::Slot<()> {
                #me::slot::Slot::root(#storage_key)
            }
        }
    });

    let all_hooks = unitify(all_hooks);
    let approve_hook = unitify(approve_hook);
    let revoke_hook = unitify(revoke_hook);
    let revoke_all_hook = unitify(revoke_all_hook);

    Ok(quote! {
        impl #imp #me::standard::nep178::Nep178ControllerInternal for #ident #ty #wher {
            type ApproveHook = (#approve_hook, #all_hooks);
            type RevokeHook = (#revoke_hook, #all_hooks);
            type RevokeAllHook = (#revoke_all_hook, #all_hooks);

            #root
        }

        #[#near_sdk::near]
        impl #imp #me::standard::nep178::Nep178 for #ident #ty #wher {
            #[payable]
            fn nft_approve(
                &mut self,
                token_id: #me::standard::nep171::TokenId,
                account_id: #near_sdk::AccountId,
                msg: Option<String>,
            ) -> #near_sdk::PromiseOrValue<()> {
                use #me::standard::nep178::*;

                #me::utils::assert_nonzero_deposit();

                let predecessor = #near_sdk::env::predecessor_account_id();

                let action = action::Nep178Approve {
                    token_id: token_id.clone(),
                    current_owner_id: predecessor.clone().into(),
                    account_id: account_id.clone().into(),
                };

                let approval_id = Nep178Controller::approve(self, &action)
                    .unwrap_or_else(|e| #near_sdk::env::panic_str(&e.to_string()));

                msg.map_or(#near_sdk::PromiseOrValue::Value(()), |msg| {
                    ext_nep178_receiver::ext(account_id)
                        .nft_on_approve(token_id, predecessor, approval_id, msg)
                        .into()
                })
            }

            #[payable]
            fn nft_revoke(
                &mut self,
                token_id: #me::standard::nep171::TokenId,
                account_id: #near_sdk::AccountId,
            ) {
                use #me::standard::nep178::*;

                #near_sdk::assert_one_yocto();

                let predecessor = #near_sdk::env::predecessor_account_id();

                let action = action::Nep178Revoke {
                    token_id,
                    current_owner_id: predecessor.into(),
                    account_id: account_id.into(),
                };

                Nep178Controller::revoke(self, &action)
                    .unwrap_or_else(|e| #near_sdk::env::panic_str(&e.to_string()));
            }

            #[payable]
            fn nft_revoke_all(&mut self, token_id: #me::standard::nep171::TokenId) {
                use #me::standard::nep178::*;

                #near_sdk::assert_one_yocto();

                let predecessor = #near_sdk::env::predecessor_account_id();

                let action = action::Nep178RevokeAll {
                    token_id,
                    current_owner_id: predecessor.into(),
                };

                Nep178Controller::revoke_all(self, &action)
                    .unwrap_or_else(|e| #near_sdk::env::panic_str(&e.to_string()));
            }

            fn nft_is_approved(
                &self,
                token_id: #me::standard::nep171::TokenId,
                approved_account_id: #near_sdk::AccountId,
                approval_id: Option<#me::standard::nep178::ApprovalId>,
            ) -> bool {
                match (
                    #me::standard::nep178::Nep178Controller::get_approval_id_for(
                        self,
                        &token_id,
                        &approved_account_id,
                    ),
                    approval_id,
                ) {
                    (Some(saved_approval_id), Some(provided_approval_id)) => saved_approval_id == provided_approval_id,
                    (Some(_), _) => true,
                    _ => false,
                }
            }
        }
    })
}
