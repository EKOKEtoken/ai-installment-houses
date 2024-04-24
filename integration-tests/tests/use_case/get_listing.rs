use integration_tests::actor::{alice, bob};
use integration_tests::client::{LedgerClient, SwapClient};
use integration_tests::{Helper, TestEnv};
use serial_test::serial;

#[test]
#[serial]
fn test_should_not_get_listing_if_owner_changed() {
    let env = TestEnv::init();

    let owner = alice();
    let my_nft = Helper::list_nft(&env, owner);

    let swap_client = SwapClient::new(&env);

    assert!(swap_client
        .get_listing(bob(), my_nft.clone())
        .unwrap()
        .is_some());

    let ledger_client = LedgerClient::new(&env);
    assert!(ledger_client
        .transfer(alice(), bob(), my_nft.clone())
        .is_ok());

    assert!(swap_client.get_listing(bob(), my_nft).unwrap().is_none());
}

#[test]
#[serial]
fn test_should_not_get_listing_if_operator_changed() {
    let env = TestEnv::init();

    let owner = alice();
    let my_nft = Helper::list_nft(&env, owner);

    let swap_client = SwapClient::new(&env);

    assert!(swap_client
        .get_listing(bob(), my_nft.clone())
        .unwrap()
        .is_some());

    let ledger_client = LedgerClient::new(&env);
    assert!(ledger_client
        .approve(alice(), bob(), my_nft.clone())
        .is_ok());

    assert!(swap_client.get_listing(bob(), my_nft).unwrap().is_none());
}
