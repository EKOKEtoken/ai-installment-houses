//! # DIP721 canister

use candid::{candid_method, Nat, Principal};
use did::CanisterInitData;
use ic_cdk_macros::{init, post_upgrade, query, update};

mod app;
pub mod did;
mod inspect;
mod storable;
mod utils;

use app::App;

#[init]
pub fn init(init_data: CanisterInitData) {
    App::init(init_data);
}

#[post_upgrade]
pub fn post_upgrade() {
    App::post_upgrade();
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
