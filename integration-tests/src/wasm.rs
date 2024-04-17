use std::path::Path;

use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Icrc2InitArgs {
    pub accounts: Vec<(Account, Nat)>,
    pub decimals: u8,
    pub fee: u64,
    pub logo: String,
    pub minting_account: Account,
    pub name: String,
    pub symbol: String,
    pub total_supply: Nat,
}

pub enum Canister {
    Icrc2,
    Ledger,
    Swap,
}

impl Canister {
    pub fn as_path(&self) -> &'static Path {
        match self {
            Canister::Ledger => Path::new("../.dfx/local/canisters/ledger/ledger.wasm"),
            Canister::Swap => Path::new("../.dfx/local/canisters/Swap/Swap.wasm"),
            Canister::Icrc2 => Path::new("../assets/wasm/icrc2-template-canister.wasm"),
        }
    }
}
