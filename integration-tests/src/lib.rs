pub mod actor;
pub mod client;
mod helper;
mod wasm;

use std::io::Read as _;
use std::path::PathBuf;

use actor::admin;
use candid::{CandidType, Decode, Encode, Nat, Principal};
use did::ledger::CanisterInitData as LedgerInitData;
use did::swap::CanisterInitData as SwapInitData;
use dip721_rs::SupportedInterface;
pub use helper::Helper;
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::{PocketIc, WasmResult};
use serde::de::DeserializeOwned;
use wasm::Canister;

use crate::wasm::Icrc2InitArgs;

const DEFAULT_CYCLES: u128 = 2_000_000_000_000_000;
pub const SWAP_INIT_BALANCE: u64 = 1_000_000_000_000_000;
pub const SWAP_DEFAULT_ROYALTY: u64 = 5;
pub const ICP_LEDGER_FEE: u64 = 10;

/// Test environment
pub struct TestEnv {
    pub pic: PocketIc,
    pub icp_ledger_id: Principal,
    pub ledger_id: Principal,
    pub swap_id: Principal,
}

impl TestEnv {
    pub fn query<R>(
        &self,
        canister: Principal,
        caller: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> anyhow::Result<R>
    where
        R: DeserializeOwned + CandidType,
    {
        let result = match self.pic.query_call(canister, caller, method, payload) {
            Ok(result) => result,
            Err(e) => anyhow::bail!("Error calling {}: {:?}", method, e),
        };
        let reply = match result {
            WasmResult::Reply(r) => r,
            WasmResult::Reject(r) => anyhow::bail!("{} was rejected: {:?}", method, r),
        };
        let ret_type = Decode!(&reply, R)?;

        Ok(ret_type)
    }

    pub fn update<R>(
        &self,
        canister: Principal,
        caller: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> anyhow::Result<R>
    where
        R: DeserializeOwned + CandidType,
    {
        let result = match self.pic.update_call(canister, caller, method, payload) {
            Ok(result) => result,
            Err(e) => anyhow::bail!("Error calling {}: {:?}", method, e),
        };

        let reply = match result {
            WasmResult::Reply(r) => r,
            WasmResult::Reject(r) => anyhow::bail!("{} was rejected: {:?}", method, r),
        };
        let ret_type = Decode!(&reply, R)?;

        Ok(ret_type)
    }

    /// Install the canisters needed for the tests
    pub fn init() -> TestEnv {
        let pic = PocketIc::new();

        // create canisters
        let icp_ledger_id = pic.create_canister();
        let ledger_id = pic.create_canister();
        let swap_id = pic.create_canister();

        // install deferred canister
        Self::install_icrc2(&pic, icp_ledger_id, swap_id, "ICP", "Internet Computer", 8);

        Self::install_ledger(&pic, ledger_id);
        Self::install_swap(&pic, swap_id, ledger_id, icp_ledger_id);

        TestEnv {
            pic,
            icp_ledger_id,
            ledger_id,
            swap_id,
        }
    }

    fn install_ledger(pic: &PocketIc, ledger_id: Principal) {
        pic.add_cycles(ledger_id, DEFAULT_CYCLES);
        let wasm_bytes = Self::load_wasm(Canister::Ledger);

        let init_arg = LedgerInitData {
            custodians: vec![admin()],
            supported_interfaces: vec![
                SupportedInterface::TransactionHistory,
                SupportedInterface::Mint,
                SupportedInterface::Approval,
                SupportedInterface::Burn,
            ],
            name: "AI Installment Houses".to_string(),
            symbol: "AIHOUSE".to_string(),
            logo: None,
        };
        let init_arg = Encode!(&init_arg).unwrap();

        pic.install_canister(ledger_id, wasm_bytes, init_arg, None);
    }

    fn install_swap(
        pic: &PocketIc,
        ledger_id: Principal,
        ledger_canister_id: Principal,
        icp_ledger_canister_id: Principal,
    ) {
        pic.add_cycles(ledger_id, DEFAULT_CYCLES);
        let wasm_bytes = Self::load_wasm(Canister::Swap);

        let init_arg = SwapInitData {
            custodians: vec![admin()],
            ledger_canister_id,
            icp_ledger_canister_id,
            sale_royalty: SWAP_DEFAULT_ROYALTY,
        };
        let init_arg = Encode!(&init_arg).unwrap();

        pic.install_canister(ledger_id, wasm_bytes, init_arg, None);
    }

    fn install_icrc2(
        pic: &PocketIc,
        id: Principal,
        swap_canister_id: Principal,
        symbol: &str,
        name: &str,
        decimals: u8,
    ) {
        pic.add_cycles(id, DEFAULT_CYCLES);
        let wasm_bytes = Self::load_wasm(Canister::Icrc2);
        let init_arg = Encode!(&Icrc2InitArgs {
            name: name.to_string(),
            symbol: symbol.to_string(),
            decimals,
            fee: ICP_LEDGER_FEE,
            logo: "https://ic0.app/img/logo.png".to_string(),
            minting_account: actor::minting_account(),
            total_supply: Nat::from(1_000_000_000_000_000_000_u64),
            accounts: vec![
                (actor::alice_account(), Nat::from(1_000_000_000_000_000_u64)),
                (actor::bob_account(), Nat::from(1_000_000_000_000_000_u64)),
                (
                    actor::charlie_account(),
                    Nat::from(1_000_000_000_000_000_u64)
                ),
                (
                    Account::from(swap_canister_id),
                    Nat::from(SWAP_INIT_BALANCE)
                )
            ],
        })
        .unwrap();

        pic.install_canister(id, wasm_bytes, init_arg, None);
    }

    fn load_wasm(canister: Canister) -> Vec<u8> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(canister.as_path());

        let mut file = std::fs::File::open(path).unwrap();
        let mut wasm_bytes = Vec::new();
        file.read_to_end(&mut wasm_bytes).unwrap();

        wasm_bytes
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        // NOTE: execute test one by one
        for tempdir in std::fs::read_dir(std::path::Path::new("/tmp")).unwrap() {
            let tempdir = tempdir.unwrap();
            if tempdir.file_name().to_string_lossy().starts_with(".tmp") {
                std::fs::remove_dir_all(tempdir.path()).unwrap();
            }
        }
    }
}
