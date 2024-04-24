use candid::{Encode, Nat, Principal};
use did::swap::{Listing, SwapResult};
use dip721_rs::TokenIdentifier;
use icrc_ledger_types::icrc1::account::Subaccount;

use crate::TestEnv;

pub struct SwapClient<'a> {
    pub env: &'a TestEnv,
}

impl<'a> SwapClient<'a> {
    pub fn new(env: &'a TestEnv) -> Self {
        Self { env }
    }

    pub fn admin_withdraw(
        &self,
        caller: Principal,
        withdraw_to: Principal,
        subaccount: Option<Subaccount>,
        amount: Nat,
    ) -> SwapResult<()> {
        self.env
            .update(
                self.env.swap_id,
                caller,
                "admin_withdraw",
                Encode!(&withdraw_to, &subaccount, &amount).unwrap(),
            )
            .expect("update failed")
    }

    pub fn list(
        &self,
        caller: Principal,
        token: TokenIdentifier,
        icp_price: Nat,
        expiration_ns: u64,
        subaccount: Option<Subaccount>,
    ) -> SwapResult<()> {
        self.env
            .update(
                self.env.swap_id,
                caller,
                "list",
                Encode!(&token, &icp_price, &expiration_ns, &subaccount).unwrap(),
            )
            .expect("update failed")
    }

    pub fn unlist(&self, caller: Principal, token: TokenIdentifier) -> SwapResult<()> {
        self.env
            .update(self.env.swap_id, caller, "unlist", Encode!(&token).unwrap())
            .expect("update failed")
    }

    pub fn get_listing(
        &self,
        caller: Principal,
        token: TokenIdentifier,
    ) -> SwapResult<Option<Listing>> {
        self.env
            .query(
                self.env.swap_id,
                caller,
                "get_listing",
                Encode!(&token).unwrap(),
            )
            .expect("update failed")
    }

    pub fn buy(
        &self,
        caller: Principal,
        token_identifier: TokenIdentifier,
        subaccount: Option<Subaccount>,
    ) -> SwapResult<Nat> {
        self.env
            .update(
                self.env.swap_id,
                caller,
                "buy",
                Encode!(&token_identifier, &subaccount).unwrap(),
            )
            .expect("update failed")
    }
}
