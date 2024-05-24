use candid::Nat;
use integration_tests::actor::{alice, alice_account, bob};
use integration_tests::client::{LedgerClient, SwapClient};
use integration_tests::{Helper, TestEnv};
use serial_test::serial;

#[test]
#[serial]
fn test_should_list_nft_for_sale() {
    let env = TestEnv::init();

    let owner = alice();
    let my_nft = Helper::mint_nft(&env, owner);

    // set swap as operator of the nft
    let ledger_client = LedgerClient::new(&env);
    assert!(ledger_client
        .approve(alice(), env.swap_id, my_nft.clone())
        .is_ok());

    let swap_client = SwapClient::new(&env);
    let icp_price = Nat::from(100u64);
    let expiration_ns = Helper::time() * 2;

    assert!(swap_client
        .list(
            alice(),
            my_nft.clone(),
            icp_price.clone(),
            expiration_ns,
            None
        )
        .is_ok());

    let listing = swap_client
        .get_listing(bob(), my_nft.clone())
        .unwrap()
        .unwrap();

    assert_eq!(listing.listing.seller, alice_account());
    assert_eq!(listing.listing.expiration_ns, expiration_ns);
    assert_eq!(listing.listing.icp_price, icp_price);
}

#[test]
#[serial]
fn test_should_fail_to_list_unowned_token() {
    let env = TestEnv::init();

    let owner = alice();
    let my_nft = Helper::mint_nft(&env, owner);

    // set swap as operator of the nft
    let ledger_client = LedgerClient::new(&env);
    assert!(ledger_client
        .approve(alice(), env.swap_id, my_nft.clone())
        .is_ok());

    let swap_client = SwapClient::new(&env);
    let icp_price = Nat::from(100u64);
    let expiration_ns = Helper::time() * 2;
    assert!(swap_client
        .list(
            bob(),
            my_nft.clone(),
            icp_price.clone(),
            expiration_ns,
            None
        )
        .is_err());
}

#[test]
#[serial]
fn test_should_fail_to_list_expiration_too_early() {
    let env = TestEnv::init();

    let owner = alice();
    let my_nft = Helper::mint_nft(&env, owner);

    // set swap as operator of the nft
    let ledger_client = LedgerClient::new(&env);
    assert!(ledger_client
        .approve(alice(), env.swap_id, my_nft.clone())
        .is_ok());

    let swap_client = SwapClient::new(&env);
    let icp_price = Nat::from(100u64);
    let expiration_ns = Helper::time() + 500_000_000_000;
    assert!(swap_client
        .list(
            bob(),
            my_nft.clone(),
            icp_price.clone(),
            expiration_ns,
            None
        )
        .is_err());
}

#[test]
#[serial]
fn test_should_fail_to_list_not_operator() {
    let env = TestEnv::init();

    let owner = alice();
    let my_nft = Helper::mint_nft(&env, owner);

    let swap_client = SwapClient::new(&env);
    let icp_price = Nat::from(100u64);
    let expiration_ns = Helper::time() * 2;

    assert!(swap_client
        .list(
            alice(),
            my_nft.clone(),
            icp_price.clone(),
            expiration_ns,
            None
        )
        .is_err());
}
