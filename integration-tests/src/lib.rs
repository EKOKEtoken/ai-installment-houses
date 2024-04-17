use candid::Principal;
use pocket_ic::PocketIc;

const DEFAULT_CYCLES: u128 = 2_000_000_000_000_000;

/// Test environment
pub struct TestEnv {
    pub pic: PocketIc,
    pub ledger_id: Principal,
    pub swap_id: Principal,
}
