use candid::Nat;
use integration_tests::actor::{admin, alice};
use integration_tests::client::SwapClient;
use integration_tests::TestEnv;

#[test]
#[serial_test::serial]
fn test_should_inspect_is_admin() {
    let env = TestEnv::init();
    let swap_client = SwapClient::new(&env);

    assert!(swap_client
        .admin_withdraw(admin(), alice(), None, Nat::from(10u64))
        .is_ok());
}

#[test]
#[serial_test::serial]
#[should_panic]
fn test_should_fail_inspect_is_admin() {
    let env = TestEnv::init();
    let swap_client = SwapClient::new(&env);

    assert!(swap_client
        .admin_withdraw(alice(), alice(), None, Nat::from(10u64))
        .is_ok());
}
