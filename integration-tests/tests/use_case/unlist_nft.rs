use integration_tests::actor::{alice, bob};
use integration_tests::client::SwapClient;
use integration_tests::{Helper, TestEnv};
use serial_test::serial;

#[test]
#[serial]
fn test_should_unlist_nft_for_sale() {
    let env = TestEnv::init();

    let owner = alice();
    let my_nft = Helper::list_nft(&env, owner);

    let swap_client = SwapClient::new(&env);

    assert!(swap_client
        .get_listing(bob(), my_nft.clone())
        .unwrap()
        .is_some());

    assert!(swap_client.unlist(alice(), my_nft.clone()).is_ok());

    assert!(swap_client.get_listing(bob(), my_nft).unwrap().is_none());
}

#[test]
#[serial]
fn test_should_not_unlist_unowned_nft() {
    let env = TestEnv::init();

    let owner = alice();
    let my_nft = Helper::list_nft(&env, owner);

    let swap_client = SwapClient::new(&env);

    assert!(swap_client
        .get_listing(bob(), my_nft.clone())
        .unwrap()
        .is_some());

    assert!(swap_client.unlist(bob(), my_nft.clone()).is_err());

    assert!(swap_client.get_listing(bob(), my_nft).unwrap().is_some());
}

#[test]
#[serial]
fn test_should_not_unlist_unlisted_nft() {
    let env = TestEnv::init();

    let owner = alice();
    let my_nft = Helper::mint_nft(&env, owner);

    let swap_client = SwapClient::new(&env);

    assert!(swap_client
        .get_listing(bob(), my_nft.clone())
        .unwrap()
        .is_none());

    assert!(swap_client.unlist(alice(), my_nft).is_err());
}
