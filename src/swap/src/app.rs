//! # App
//!
//! API for App

mod client;
mod configuration;
pub mod inspect;
mod memory;
mod storage;
#[cfg(test)]
pub mod test_utils;

use candid::{Nat, Principal};
use did::swap::{CanisterInitData, Listing, SwapError, SwapResult};
use dip721_rs::{TokenIdentifier, TokenMetadata};
use icrc_ledger_types::icrc1::account::{Account, Subaccount};

use self::client::{Dip721Client, IcrcLedgerClient};
use self::configuration::Configuration;
use self::inspect::Inspect;
use self::storage::Storage;
use crate::utils::{caller, canister_id, cycles, time};

#[derive(Default)]
/// App canister
pub struct App;

impl App {
    /// On init set custodians and canisters ids
    pub fn init(init_data: CanisterInitData) {
        Configuration::set_custodians(&init_data.custodians);
        Configuration::set_ledger_canister(init_data.ledger_canister_id);
        Configuration::set_icp_ledger_canister(init_data.icp_ledger_canister_id);
        Configuration::set_sale_royalty(init_data.sale_royalty);
    }

    /// Set ledger canister
    pub fn admin_set_ledger_canister(ledger_canister_id: Principal) {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized call to admin_set_ledger_canister");
        }

        Configuration::set_ledger_canister(ledger_canister_id);
    }

    /// Set sale royalty
    pub fn admin_set_sale_royalty(sale_royalty: u64) {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized call to admin_set_sale_royalty");
        }

        Configuration::set_sale_royalty(sale_royalty);
    }

    /// Set custodians
    pub fn admin_set_custodians(custodians: Vec<Principal>) {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized call to admin_set_custodians");
        }

        Configuration::set_custodians(&custodians);
    }

    /// Get the cycles available to the canister
    pub fn admin_cycles() -> Nat {
        if Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized call to admin_cycles");
        }
        cycles()
    }

    /// Withdraw ICP to the provided account
    pub async fn admin_withdraw(
        withdraw_to: Principal,
        subaccount: Option<Subaccount>,
        amount: Nat,
    ) -> SwapResult<()> {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized call to admin_withdraw");
        }

        let icp_ledger_client = IcrcLedgerClient::new(Configuration::get_icp_ledger_canister());
        icp_ledger_client
            .icrc1_transfer(
                Account {
                    owner: withdraw_to,
                    subaccount,
                },
                amount,
            )
            .await
            .map_err(|(code, _)| SwapError::from(code))??;

        Ok(())
    }

    /// List a token for sale
    pub async fn list(
        token_identifier: TokenIdentifier,
        icp_price: Nat,
        expiration_ns: u64,
        subaccount: Option<Subaccount>,
    ) -> SwapResult<()> {
        let token = Self::get_token_metadata(&token_identifier).await?;
        Storage::list(token, icp_price, expiration_ns, subaccount)
    }

    /// Unlist a token
    pub async fn unlist(token_identifier: TokenIdentifier) -> SwapResult<()> {
        let token = Self::get_token_metadata(&token_identifier).await?;
        Storage::unlist(token)
    }

    /// Get listing
    pub async fn get_listing(token_identifier: TokenIdentifier) -> SwapResult<Option<Listing>> {
        let token = Self::get_token_metadata(&token_identifier).await?;
        Ok(Storage::get_listing(token))
    }

    /// Buy a token
    ///
    /// Returns the transfer txid from the NFT ledger
    pub async fn buy(
        token_identifier: TokenIdentifier,
        subaccount: Option<Subaccount>,
    ) -> SwapResult<Nat> {
        // get listing
        let token = Self::get_token_metadata(&token_identifier).await?;
        let listing = Storage::get_listing(token.clone()).ok_or(SwapError::NoSuchListing)?;

        // get amounts
        let icp_price = listing.icp_price;
        let sale_royalty = Self::sale_royalty(icp_price.clone());
        let seller_amount = icp_price.clone() - sale_royalty.clone();

        let icp_ledger_client = IcrcLedgerClient::new(Configuration::get_icp_ledger_canister());
        // get canister fee and caller's balance
        let caller_account = Account {
            owner: caller(),
            subaccount,
        };
        let caller_balance = icp_ledger_client
            .icrc1_balance_of(caller_account)
            .await
            .map_err(|(code, _)| SwapError::from(code))?;
        let caller_allowance = icp_ledger_client
            .icrc2_allowance(Self::canister_account(), caller_account)
            .await
            .map_err(|(code, _)| SwapError::from(code))?;
        let icrc_fee = icp_ledger_client
            .icrc1_fee()
            .await
            .map_err(|(code, _)| SwapError::from(code))?;

        // check if caller has enough balance and allowance
        let total_amount_to_pay = icp_price.clone() + icrc_fee.clone() + icrc_fee;
        if caller_balance < total_amount_to_pay {
            return Err(SwapError::InsufficientBalance(
                caller_balance,
                total_amount_to_pay,
            ));
        }
        if caller_allowance.allowance < icp_price.clone() {
            return Err(SwapError::InsufficientAllowance(
                caller_allowance.allowance,
                icp_price,
            ));
        }
        // check whether allowance has expired
        if caller_allowance
            .expires_at
            .map(|expiration| expiration < time())
            .unwrap_or_default()
        {
            return Err(SwapError::AllowanceExpired);
        }

        // transfer ICP without fee to seller
        let seller_account = listing.seller;
        Self::icp_transfer_from(
            &icp_ledger_client,
            caller_account,
            seller_account,
            seller_amount,
        )
        .await?;
        // transfer fee to canister
        Self::icp_transfer_from(
            &icp_ledger_client,
            caller_account,
            Self::canister_account(),
            sale_royalty,
        )
        .await?;

        // transfer the NFT
        let dip721_client = Dip721Client::new(Configuration::get_ledger_canister());
        let tx_id = dip721_client
            .dip721_transfer_from(seller_account.owner, caller(), &token_identifier)
            .await
            .map_err(|(code, _)| SwapError::from(code))??;

        // unlist
        Storage::remove(token.token_identifier);

        Ok(tx_id)
    }

    /// Get token metadata from ledger canister
    async fn get_token_metadata(token_identifier: &TokenIdentifier) -> SwapResult<TokenMetadata> {
        let client = Dip721Client::new(Configuration::get_ledger_canister());
        client
            .dip721_token_metadata(token_identifier)
            .await
            .map_err(|(code, _)| SwapError::from(code))?
            .map_err(|e| e.into())
    }

    /// Perform a ICRC2 transfer from op on the ICP ledger canister
    async fn icp_transfer_from(
        client: &IcrcLedgerClient,
        from: Account,
        to: Account,
        amount: Nat,
    ) -> SwapResult<Nat> {
        client
            .icrc2_transfer_from(None, from, to, amount)
            .await
            .map_err(|(code, _)| SwapError::from(code))?
            .map_err(|e| e.into())
    }

    fn sale_royalty(icp_price: Nat) -> Nat {
        icp_price * Nat::from(Configuration::get_sale_royalty()) / Nat::from(100u64)
    }

    #[inline]
    fn canister_account() -> Account {
        Account {
            owner: canister_id(),
            subaccount: None,
        }
    }
}

#[cfg(test)]
mod test {

    use test::test_utils::{alice, bob};

    use self::test_utils::{icp_ledger_canister, ledger_canister};
    use super::*;
    use crate::constants::DEFAULT_SALE_ROYALTY;

    #[test]
    fn test_should_init_canister() {
        let init_data = CanisterInitData {
            custodians: vec![caller()],
            ledger_canister_id: ledger_canister(),
            icp_ledger_canister_id: icp_ledger_canister(),
            sale_royalty: 15,
        };
        App::init(init_data);

        assert_eq!(Configuration::get_sale_royalty(), 15);
        assert_eq!(Configuration::get_custodians(), vec![caller()]);
        assert_eq!(Configuration::get_ledger_canister(), ledger_canister());
        assert_eq!(
            Configuration::get_icp_ledger_canister(),
            icp_ledger_canister()
        );
    }

    #[test]
    fn test_should_set_ledger_canister() {
        init_canister();
        let ledger_canister_id = alice();
        App::admin_set_ledger_canister(ledger_canister_id);

        assert_eq!(Configuration::get_ledger_canister(), alice());
    }

    #[test]
    fn test_should_set_sale_royalty() {
        init_canister();
        App::admin_set_sale_royalty(15);

        assert_eq!(Configuration::get_sale_royalty(), 15);
    }

    #[test]
    fn test_should_set_custodians() {
        init_canister();
        let custodians = vec![alice(), bob()];
        App::admin_set_custodians(custodians.clone());

        assert_eq!(Configuration::get_custodians(), custodians);
    }

    #[tokio::test]
    async fn test_should_list() {
        init_canister();
        let token_id = TokenIdentifier::from(1u64);
        assert!(App::list(
            token_id.clone(),
            Nat::from(10_000u64),
            time() * 2,
            Some([1; 32])
        )
        .await
        .is_ok());

        // get listing
        let listing = App::get_listing(token_id).await.unwrap().unwrap();
        assert_eq!(listing.icp_price, Nat::from(10_000u64));
        assert_eq!(
            listing.seller,
            Account {
                owner: caller(),
                subaccount: Some([1; 32])
            }
        );
    }

    #[tokio::test]
    async fn test_should_unlist() {
        init_canister();
        let token_id = TokenIdentifier::from(1u64);
        assert!(App::list(
            token_id.clone(),
            Nat::from(10_000u64),
            time() * 2,
            Some([1; 32])
        )
        .await
        .is_ok());

        assert!(App::unlist(token_id.clone()).await.is_ok());
        assert!(App::get_listing(token_id).await.unwrap().is_none());
    }
    #[tokio::test]
    async fn test_should_not_unlist() {
        init_canister();
        let token_id = TokenIdentifier::from(1u64);

        assert!(App::unlist(token_id.clone()).await.is_err());
    }

    #[tokio::test]
    async fn test_should_buy() {
        init_canister();
        let token_id = TokenIdentifier::from(1u64);
        assert!(App::list(
            token_id.clone(),
            Nat::from(10_000u64),
            time() * 2,
            Some([1; 32])
        )
        .await
        .is_ok());

        assert!(App::buy(token_id.clone(), None).await.is_ok());
        // should no more be listed
        assert!(App::get_listing(token_id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_should_not_buy() {
        init_canister();
        let token_id = TokenIdentifier::from(1u64);
        assert!(App::buy(token_id, None).await.is_err());
    }

    #[inline]
    fn init_canister() {
        let init_data = CanisterInitData {
            custodians: vec![caller()],
            ledger_canister_id: ledger_canister(),
            icp_ledger_canister_id: icp_ledger_canister(),
            sale_royalty: DEFAULT_SALE_ROYALTY,
        };
        App::init(init_data);
    }
}
