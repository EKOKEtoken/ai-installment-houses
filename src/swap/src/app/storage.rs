use std::cell::RefCell;

use candid::Nat;
use did::swap::{Listing, SwapError, SwapResult};
use dip721_rs::{TokenIdentifier, TokenMetadata};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use icrc_ledger_types::icrc1::account::{Account, Subaccount};

use crate::app::memory::{LISTINGS_MEMORY_ID, MEMORY_MANAGER};
use crate::constants::MINIMUM_LISTING_DURATION_NS;
use crate::storable::StorableNat;
use crate::utils::{caller, canister_id, time};

thread_local! {

    /// Canister custodians
    static LISTINGS: RefCell<StableBTreeMap<StorableNat, Listing, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableBTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(LISTINGS_MEMORY_ID)))
    );

}

pub struct Storage;

impl Storage {
    /// Lists a DIP721 token for sale
    pub fn list(
        token: TokenMetadata,
        icp_price: Nat,
        expiration_ns: u64,
        subaccount: Option<Subaccount>,
    ) -> SwapResult<()> {
        // check owner
        if Some(caller()) != token.owner {
            return Err(SwapError::UnauthorizedCaller);
        }
        // check operator
        if token.operator != Some(canister_id()) {
            return Err(SwapError::CanisterIsNotOperator);
        }
        // check expiration
        if expiration_ns < time() + MINIMUM_LISTING_DURATION_NS {
            return Err(SwapError::ExpirationTooEarly);
        }
        LISTINGS.with_borrow_mut(|listings| {
            listings.insert(
                token.token_identifier.into(),
                Listing {
                    seller: Account {
                        owner: caller(),
                        subaccount,
                    },
                    icp_price,
                    expiration_ns,
                },
            )
        });

        Ok(())
    }

    /// Unlists a DIP721 token
    pub fn unlist(token: TokenMetadata) -> SwapResult<()> {
        // check owner
        if Some(caller()) != token.owner {
            return Err(SwapError::UnauthorizedCaller);
        }

        LISTINGS.with_borrow_mut(|listings| {
            let key = StorableNat::from(token.token_identifier);
            if listings.contains_key(&key) {
                listings.remove(&key);
                Ok(())
            } else {
                Err(SwapError::NoSuchListing)
            }
        })
    }

    /// Force unlisting of a token after a sale.
    ///
    /// This function is used to unlist a token after a successful sale.
    /// It does not check the caller's identity. Use `unlist` for the regular unlisting process.
    pub fn remove(token_identifier: TokenIdentifier) {
        LISTINGS.with_borrow_mut(|listings| {
            let key = StorableNat::from(token_identifier);
            if listings.contains_key(&key) {
                listings.remove(&key);
            }
        });
    }

    /// Given a DIP721 token, returns its listing if it is still valid.
    pub fn get_listing(token: &TokenMetadata) -> Option<Listing> {
        LISTINGS.with_borrow(|listings| {
            match listings.get(&StorableNat::from(token.token_identifier.clone())) {
                Some(listing) if Self::is_listed(&listing, &token) => Some(listing.clone()),
                Some(_) => None,
                None => None,
            }
        })
    }

    pub fn listed_tokens() -> Vec<TokenIdentifier> {
        LISTINGS.with_borrow(|listings| listings.iter().map(|(key, _)| key.0.clone()).collect())
    }

    #[inline]
    /// Given a DIP721 token, returns whether it is listed and if its listing is still valid.
    ///
    /// The conditions are the following:
    ///
    /// - The token is listed.
    /// - The token is still owned by the listing owner.
    /// - The listing has not expired.
    /// - The canister is the operator of the token.
    fn is_listed(listing: &Listing, token: &TokenMetadata) -> bool {
        token.operator == Some(canister_id())
            && Some(listing.seller.owner) == token.owner
            && listing.expiration_ns > time()
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::{alice, bob, with_mock_token};

    #[test]
    fn test_should_list_token() {
        let token = with_mock_token(1, |token| {
            token.operator = Some(canister_id());
        });
        let icp_price = Nat::from(100u64);
        let expiration_ns = time() * 2;

        assert!(Storage::list(token.clone(), icp_price, expiration_ns, None).is_ok());
    }

    #[test]
    fn test_should_not_list_bad_owner() {
        let token = with_mock_token(1, |token| {
            token.owner = Some(bob());
        });
        let icp_price = Nat::from(100u64);
        let expiration_ns = time() * 2;

        assert_eq!(
            Storage::list(token.clone(), icp_price, expiration_ns, None),
            Err(SwapError::UnauthorizedCaller)
        );
    }

    #[test]
    fn test_should_not_list_canister_is_not_op() {
        let token = with_mock_token(1, |token| {
            token.operator = Some(alice());
        });

        let icp_price = Nat::from(100u64);
        let expiration_ns = time() * 2;

        assert_eq!(
            Storage::list(token.clone(), icp_price, expiration_ns, None),
            Err(SwapError::CanisterIsNotOperator)
        );
    }

    #[test]
    fn test_should_not_list_exp_too_early() {
        let token = with_mock_token(1, |token| {
            token.operator = Some(canister_id());
        });
        let icp_price = Nat::from(100u64);
        let expiration_ns = time() + MINIMUM_LISTING_DURATION_NS - 60_000_000_000;

        assert_eq!(
            Storage::list(token.clone(), icp_price, expiration_ns, None),
            Err(SwapError::ExpirationTooEarly)
        );
    }

    #[test]
    fn test_should_unlist_token() {
        let token = with_mock_token(1, |token| {
            token.operator = Some(canister_id());
        });

        assert!(Storage::list(token.clone(), Nat::from(100u64), time() * 2, None).is_ok());
        assert!(Storage::unlist(token.clone()).is_ok());
    }

    #[test]
    fn test_should_not_unlist_bad_owner() {
        let mut token = with_mock_token(1, |token| {
            token.operator = Some(canister_id());
        });

        assert!(Storage::list(token.clone(), Nat::from(100u64), time() * 2, None).is_ok());

        token.owner = Some(bob());

        assert_eq!(Storage::unlist(token), Err(SwapError::UnauthorizedCaller));
    }

    #[test]
    fn test_should_not_unlist_no_such_listing() {
        let token = with_mock_token(1, |token| {
            token.operator = Some(canister_id());
        });

        assert_eq!(Storage::unlist(token), Err(SwapError::NoSuchListing));
    }

    #[test]
    fn test_should_get_listing() {
        let token = with_mock_token(1, |token| {
            token.operator = Some(canister_id());
        });
        let icp_price = Nat::from(100u64);
        let expiration_ns = time() * 2;

        assert!(Storage::list(
            token.clone(),
            icp_price.clone(),
            expiration_ns,
            Some([1; 32])
        )
        .is_ok());

        let listing = Storage::get_listing(&token).unwrap();
        assert_eq!(
            listing,
            Listing {
                seller: Account {
                    owner: caller(),
                    subaccount: Some([1; 32]),
                },
                icp_price,
                expiration_ns
            }
        );
    }

    #[test]
    fn test_should_not_get_listing_if_owner_changed() {
        let token = with_mock_token(1, |token| {
            token.operator = Some(canister_id());
        });
        let icp_price = Nat::from(100u64);
        let expiration_ns = time() * 2;

        assert!(Storage::list(token.clone(), icp_price.clone(), expiration_ns, None).is_ok());

        let mut token = token.clone();
        token.owner = Some(bob());

        assert!(Storage::get_listing(&token).is_none());
    }

    #[test]
    fn test_should_not_get_listing_if_operator_changed() {
        let token = with_mock_token(1, |token| {
            token.operator = Some(canister_id());
        });
        let icp_price = Nat::from(100u64);
        let expiration_ns = time() * 2;

        assert!(Storage::list(token.clone(), icp_price.clone(), expiration_ns, None).is_ok());

        let mut token = token.clone();
        token.operator = Some(bob());

        assert!(Storage::get_listing(&token).is_none());
    }

    #[test]
    fn test_should_not_get_listing_if_expired() {
        let token = with_mock_token(1, |token| {
            token.operator = Some(canister_id());
        });
        let icp_price = Nat::from(100u64);
        let expiration_ns = time() * 2;

        assert!(Storage::list(token.clone(), icp_price.clone(), expiration_ns, None).is_ok());
        // change exp
        LISTINGS.with_borrow_mut(|listings| {
            let key = StorableNat::from(token.token_identifier.clone());
            let mut listing = listings.get(&key).unwrap();
            listing.expiration_ns = time() - 1;
            listings.insert(key, listing);
        });

        assert!(Storage::get_listing(&token).is_none());
    }
}
