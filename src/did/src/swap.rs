use candid::{CandidType, Decode, Encode, Nat, Principal};
use dip721_rs::NftError;
use ic_cdk::api::call::RejectionCode;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferError;
use icrc_ledger_types::icrc2::transfer_from::TransferFromError;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, CandidType, Deserialize, PartialEq)]
pub struct CanisterInitData {
    pub custodians: Vec<Principal>,
    pub icp_ledger_canister_id: Principal,
    pub ledger_canister_id: Principal,
    /// Sale royalty percentage
    pub sale_royalty: u64,
}

pub type SwapResult<T> = Result<T, SwapError>;

#[derive(Debug, Error, CandidType, Deserialize, PartialEq)]
pub enum SwapError {
    /// The caller is not authorized to perform the operation.
    #[error("unauthorized caller")]
    UnauthorizedCaller,
    /// The listing has expired.
    #[error("listing expired")]
    ListingExpired,
    /// The expiration timestamp is less than 24 hours.
    #[error("expiration timestamp is too early")]
    ExpirationTooEarly,
    /// the canister is not the operator of the token
    #[error("the canister is not the operator of the token")]
    CanisterIsNotOperator,
    /// The caller hasn't enough icp to allow the swap
    #[error("insufficient balance: have {0}, need {1}")]
    InsufficientBalance(Nat, Nat),
    /// The caller hasn't given enough allowance to the canister to allow the swap
    #[error("insufficient balance: have {0}, need {1}")]
    InsufficientAllowance(Nat, Nat),
    /// The allowance has expired.
    #[error("allowance expired")]
    AllowanceExpired,
    /// The listing does not exist.
    #[error("no such listing")]
    NoSuchListing,
    #[error("nft error: {0}")]
    Nft(#[from] NftError),
    #[error("ICRC transfer error: {0:?}")]
    IcrcTransfer(TransferError),
    #[error("ICRC transfer from error: {0:?}")]
    IcrcTransferFrom(TransferFromError),
    #[error("canister call error: {0:?}")]
    CanisterCall(RejectionCode),
}

impl From<RejectionCode> for SwapError {
    fn from(err: RejectionCode) -> Self {
        Self::CanisterCall(err)
    }
}

impl From<TransferError> for SwapError {
    fn from(err: TransferError) -> Self {
        Self::IcrcTransfer(err)
    }
}

impl From<TransferFromError> for SwapError {
    fn from(err: TransferFromError) -> Self {
        Self::IcrcTransferFrom(err)
    }
}

/// A listing of a nft.
#[derive(Debug, CandidType, Deserialize, PartialEq, Clone)]
pub struct Listing {
    /// The account of the nft seller.
    pub seller: Account,
    /// The price of the listing in ICP.
    pub icp_price: Nat,
    /// The timestamp in nanoseconds when the listing will expire.
    pub expiration_ns: u64,
}

impl Storable for Listing {
    const BOUND: Bound = Bound::Bounded {
        max_size: 128,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_listing_storable() {
        let listing = Listing {
            seller: Account::from(Principal::management_canister()),
            icp_price: Nat::from(100u64),
            expiration_ns: 123,
        };
        let bytes = listing.to_bytes();
        let decoded = Listing::from_bytes(bytes);
        assert_eq!(listing, decoded);

        let listing = Listing {
            seller: Account {
                owner: Principal::management_canister(),
                subaccount: Some([1u8; 32]),
            },
            icp_price: Nat::from(100u64),
            expiration_ns: 123,
        };
        let bytes = listing.to_bytes();
        let decoded = Listing::from_bytes(bytes);
        assert_eq!(listing, decoded);
    }
}
