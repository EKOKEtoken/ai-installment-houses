//! # App
//!
//! API for App

mod configuration;
mod memory;
#[cfg(test)]
mod test_utils;

use crate::did::CanisterInitData;
use crate::utils::caller;

use self::configuration::Configuration;

#[derive(Default)]
/// App canister
pub struct App;

impl App {
    /// On init set custodians and canisters ids
    pub fn init(init_data: CanisterInitData) {
        Configuration::set_custodians(&init_data.custodians);
        Configuration::set_ledger_canister(init_data.ledger_canister_id);
        Configuration::set_icp_ledger_canister(init_data.icp_ledger_canister_id);
    }

    /// Task to execute on post upgrade
    pub fn post_upgrade() {
        todo!();
    }
}
