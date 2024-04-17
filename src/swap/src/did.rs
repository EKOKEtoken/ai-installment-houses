use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize, PartialEq)]
pub struct CanisterInitData {
    pub custodians: Vec<Principal>,
    pub icp_ledger_canister_id: Principal,
    pub ledger_canister_id: Principal,
}
