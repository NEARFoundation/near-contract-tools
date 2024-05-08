//! NEP-171 actions.
//!
//! Used when calling various functions on [`Nep171Controller`]. Also used when
//! implementing [`Hook`]s for the NEP-171 component.

use std::borrow::Cow;

use super::*;

/// NEP-171 mint action.
#[derive(Clone, Debug, PartialEq, Eq)]
#[near]
pub struct Nep171Mint<'a> {
    /// Token IDs to mint.
    pub token_ids: Vec<TokenId>,
    /// Account ID of the receiver.
    pub receiver_id: Cow<'a, AccountIdRef>,
    /// Optional memo string.
    pub memo: Option<Cow<'a, str>>,
}

impl<'a> Nep171Mint<'a> {
    /// Create a new mint action.
    pub fn new(token_ids: Vec<TokenId>, receiver_id: impl Into<Cow<'a, AccountIdRef>>) -> Self {
        Self {
            token_ids,
            receiver_id: receiver_id.into(),
            memo: None,
        }
    }

    /// Add a memo string.
    #[must_use]
    pub fn memo(self, memo: impl Into<Cow<'a, str>>) -> Self {
        Self {
            memo: Some(memo.into()),
            ..self
        }
    }
}

/// NEP-171 burn action.
#[derive(Clone, Debug, PartialEq, Eq)]
#[near]
pub struct Nep171Burn<'a> {
    /// Token IDs to burn.
    pub token_ids: Vec<TokenId>,
    /// Account ID of the owner.
    pub owner_id: Cow<'a, AccountIdRef>,
    /// Optional memo string.
    pub memo: Option<Cow<'a, str>>,
}

impl<'a> Nep171Burn<'a> {
    /// Create a new burn action.
    pub fn new(token_ids: Vec<TokenId>, owner_id: impl Into<Cow<'a, AccountIdRef>>) -> Self {
        Self {
            token_ids,
            owner_id: owner_id.into(),
            memo: None,
        }
    }

    /// Add a memo string.
    #[must_use]
    pub fn memo(self, memo: impl Into<Cow<'a, str>>) -> Self {
        Self {
            memo: Some(memo.into()),
            ..self
        }
    }
}

/// Transfer metadata generic over both types of transfer (`nft_transfer` and
/// `nft_transfer_call`).
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
#[near]
pub struct Nep171Transfer<'a> {
    /// Why is this sender allowed to perform this transfer?
    pub authorization: Nep171TransferAuthorization,
    /// Sending account ID.
    pub sender_id: Cow<'a, AccountIdRef>,
    /// Receiving account ID.
    pub receiver_id: Cow<'a, AccountIdRef>,
    /// Token ID.
    pub token_id: TokenId,
    /// Optional memo string.
    pub memo: Option<Cow<'a, str>>,
    /// Message passed to contract located at `receiver_id` in the case of `nft_transfer_call`.
    pub msg: Option<Cow<'a, str>>,
    /// `true` if the transfer is a revert for a `nft_transfer_call`.
    pub revert: bool,
}

impl<'a> Nep171Transfer<'a> {
    /// Create a new transfer action.
    pub fn new(
        token_id: TokenId,
        sender_id: impl Into<Cow<'a, AccountIdRef>>,
        receiver_id: impl Into<Cow<'a, AccountIdRef>>,
        authorization: Nep171TransferAuthorization,
    ) -> Self {
        Self {
            authorization,
            sender_id: sender_id.into(),
            receiver_id: receiver_id.into(),
            token_id,
            memo: None,
            msg: None,
            revert: false,
        }
    }

    /// Add a memo string.
    #[must_use]
    pub fn memo(self, memo: impl Into<Cow<'a, str>>) -> Self {
        Self {
            memo: Some(memo.into()),
            ..self
        }
    }

    /// Add a message string.
    #[must_use]
    pub fn msg(self, msg: impl Into<Cow<'a, str>>) -> Self {
        Self {
            msg: Some(msg.into()),
            ..self
        }
    }
}
