//! # DIP721 canister

use candid::{candid_method, Nat, Principal};
use did::http::{HttpRequest, HttpResponse};
use did::swap::{CanisterInitData, GetListing, SwapResult};
use dip721_rs::TokenIdentifier;
use http::HttpApi;
use ic_cdk_macros::{init, query, update};

mod app;
mod constants;
mod http;
mod inspect;
mod storable;
mod utils;

use app::App;
use icrc_ledger_types::icrc1::account::Subaccount;

#[init]
pub fn init(init_data: CanisterInitData) {
    App::init(init_data);
}

#[query]
#[candid_method(query)]
pub fn admin_cycles() -> Nat {
    App::admin_cycles()
}

#[update]
#[candid_method(update)]
pub fn admin_set_ledger_canister(ledger_canister_id: Principal) {
    App::admin_set_ledger_canister(ledger_canister_id);
}

#[update]
#[candid_method(update)]
pub fn admin_set_sale_royalty(sale_royalty: u64) {
    App::admin_set_sale_royalty(sale_royalty);
}

#[update]
#[candid_method(update)]
pub fn admin_set_custodians(custodians: Vec<Principal>) {
    App::admin_set_custodians(custodians);
}

#[update]
#[candid_method(update)]
pub async fn admin_withdraw(
    withdraw_to: Principal,
    subaccount: Option<Subaccount>,
    amount: Nat,
) -> SwapResult<()> {
    App::admin_withdraw(withdraw_to, subaccount, amount).await
}

#[update]
#[candid_method(update)]
pub async fn list(
    token: TokenIdentifier,
    icp_price: Nat,
    expiration_ns: u64,
    subaccount: Option<Subaccount>,
) -> SwapResult<()> {
    App::list(token, icp_price, expiration_ns, subaccount).await
}

#[update]
#[candid_method(update)]
pub async fn unlist(token_identifier: TokenIdentifier) -> SwapResult<()> {
    App::unlist(token_identifier).await
}

#[query]
#[candid_method(query)]
pub async fn get_listing(token_identifier: TokenIdentifier) -> SwapResult<Option<GetListing>> {
    App::get_listing(token_identifier).await
}

#[update]
#[candid_method(update)]
pub async fn buy(
    token_identifier: TokenIdentifier,
    subaccount: Option<Subaccount>,
) -> SwapResult<Nat> {
    App::buy(token_identifier, subaccount).await
}

// HTTP endpoint
#[query]
#[candid_method(query)]
pub async fn http_request(req: HttpRequest) -> HttpResponse {
    HttpApi::handle_http_request(req).await
}

#[allow(dead_code)]
fn main() {
    // The line below generates did types and service definition from the
    // methods annotated with `candid_method` above. The definition is then
    // obtained with `__export_service()`.
    candid::export_service!();
    std::print!("{}", __export_service());
}

/// GetRandom fixup to allow getrandom compilation.
/// A getrandom implementation that always fails
///
/// This is a workaround for the fact that the `getrandom` crate does not compile
/// for the `wasm32-unknown-ic` target. This is a dummy implementation that always
/// fails with `Error::UNSUPPORTED`.
pub fn getrandom_always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}

getrandom::register_custom_getrandom!(getrandom_always_fail);
