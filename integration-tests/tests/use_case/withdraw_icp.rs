use candid::Nat;
use icrc_ledger_types::icrc1::account::Account;
use integration_tests::actor::{admin, alice, alice_account};
use integration_tests::client::{IcrcLedgerClient, SwapClient};
use integration_tests::{TestEnv, ICP_LEDGER_FEE};
use serial_test::serial;

#[test]
#[serial]
fn test_should_withdraw_icp() {
    let env = TestEnv::init();
    let alice = alice();
    let alice_account = alice_account();
    let swap_account = Account::from(env.swap_id);
    let icp_ledger_client = IcrcLedgerClient::new(env.icp_ledger_id, &env);

    let init_alice_balance = icp_ledger_client.icrc1_balance_of(alice_account);
    let init_canister_balance = icp_ledger_client.icrc1_balance_of(swap_account);

    let amount = Nat::from(500_000u64);
    let swap_client = SwapClient::new(&env);
    assert!(swap_client
        .admin_withdraw(admin(), alice, alice_account.subaccount, amount.clone())
        .is_ok());

    let final_alice_balance = icp_ledger_client.icrc1_balance_of(alice_account);
    let final_canister_balance = icp_ledger_client.icrc1_balance_of(swap_account);

    assert_eq!(final_alice_balance, init_alice_balance + amount.clone());
    assert_eq!(
        final_canister_balance,
        init_canister_balance - amount - ICP_LEDGER_FEE
    );
}
