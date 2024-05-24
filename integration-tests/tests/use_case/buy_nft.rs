use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use integration_tests::actor::{alice, alice_account, bob, bob_account};
use integration_tests::client::{IcrcLedgerClient, LedgerClient, SwapClient};
use integration_tests::{Helper, TestEnv, ICP_LEDGER_FEE, SWAP_DEFAULT_ROYALTY};
use serial_test::serial;

#[test]
#[serial]
fn test_should_buy_nft() {
    let env = TestEnv::init();

    let my_nft = Helper::list_nft(&env, alice());

    let swap_client = SwapClient::new(&env);
    let icp_ledger_client = IcrcLedgerClient::new(env.icp_ledger_id, &env);
    let ledger_client = LedgerClient::new(&env);
    // get init balances
    let alice_init_balance = icp_ledger_client.icrc1_balance_of(alice_account());
    let bob_init_balance = icp_ledger_client.icrc1_balance_of(bob_account());
    let swap_init_balance = icp_ledger_client.icrc1_balance_of(Account::from(env.swap_id));
    // get listing
    let listing = swap_client
        .get_listing(alice(), my_nft.clone())
        .unwrap()
        .unwrap();
    let sale_royalty = listing.listing.icp_price.clone() * SWAP_DEFAULT_ROYALTY / 100u64;
    let icp_price_wno_royalty = listing.listing.icp_price.clone() - sale_royalty.clone();

    // bob buys the nft; first give to the canister the approval to spend icp_price
    assert!(icp_ledger_client
        .icrc2_approve(
            bob(),
            Account::from(env.swap_id),
            listing.listing.icp_price.clone() + ICP_LEDGER_FEE + ICP_LEDGER_FEE,
            bob_account().subaccount
        )
        .is_ok());
    // buy
    assert!(swap_client
        .buy(bob(), my_nft.clone(), bob_account().subaccount)
        .is_ok());

    // verify balances
    let alice_final_balance = icp_ledger_client.icrc1_balance_of(alice_account());
    let bob_final_balance = icp_ledger_client.icrc1_balance_of(bob_account());
    let swap_final_balance = icp_ledger_client.icrc1_balance_of(Account::from(env.swap_id));
    assert_eq!(
        alice_final_balance,
        alice_init_balance + icp_price_wno_royalty
    );
    assert_eq!(
        bob_final_balance,
        bob_init_balance
            - listing.listing.icp_price
            - ICP_LEDGER_FEE
            - ICP_LEDGER_FEE
            - ICP_LEDGER_FEE // 3 because of approve
    );
    assert_eq!(swap_final_balance, swap_init_balance + sale_royalty);

    // verify ownership
    assert_eq!(
        ledger_client.owner_of(my_nft.clone()).unwrap().unwrap(),
        bob()
    );
    // should no more be listed
    assert!(swap_client.get_listing(bob(), my_nft).unwrap().is_none());
}

#[test]
#[serial]
fn test_should_not_buy_nft_if_not_enough_balance() {
    let env = TestEnv::init();

    let my_nft = Helper::list_nft(&env, alice());

    let swap_client = SwapClient::new(&env);
    let icp_ledger_client = IcrcLedgerClient::new(env.icp_ledger_id, &env);
    let ledger_client = LedgerClient::new(&env);
    // get init balances
    let alice_init_balance = icp_ledger_client.icrc1_balance_of(alice_account());
    let bob_init_balance = icp_ledger_client.icrc1_balance_of(bob_account());
    let swap_init_balance = icp_ledger_client.icrc1_balance_of(Account::from(env.swap_id));
    // get listing
    let listing = swap_client
        .get_listing(alice(), my_nft.clone())
        .unwrap()
        .unwrap();

    // bob buys the nft; first give to the canister the approval to spend icp_price
    assert!(icp_ledger_client
        .icrc2_approve(
            bob(),
            Account::from(env.swap_id),
            listing.listing.icp_price.clone() + ICP_LEDGER_FEE + ICP_LEDGER_FEE,
            bob_account().subaccount
        )
        .is_ok());
    // transfer bob balance to alice
    let amt_to_transfer = bob_init_balance - listing.listing.icp_price - 1_u64;

    assert!(icp_ledger_client
        .icrc1_transfer(
            bob(),
            Principal::management_canister().into(),
            amt_to_transfer,
            bob_account().subaccount
        )
        .is_ok());

    // buy
    assert!(swap_client
        .buy(bob(), my_nft.clone(), bob_account().subaccount)
        .is_err());

    // verify ownership
    assert_eq!(
        ledger_client.owner_of(my_nft.clone()).unwrap().unwrap(),
        alice()
    );
    // should no more be listed
    assert!(swap_client.get_listing(bob(), my_nft).unwrap().is_some());

    let alice_final_balance = icp_ledger_client.icrc1_balance_of(alice_account());
    let swap_final_balance = icp_ledger_client.icrc1_balance_of(Account::from(env.swap_id));
    assert_eq!(alice_init_balance, alice_final_balance);
    assert_eq!(swap_init_balance, swap_final_balance);
}

#[test]
#[serial]
fn test_should_not_buy_nft_if_not_enough_allowance() {
    let env = TestEnv::init();

    let my_nft = Helper::list_nft(&env, alice());

    let swap_client = SwapClient::new(&env);
    let icp_ledger_client = IcrcLedgerClient::new(env.icp_ledger_id, &env);
    let ledger_client = LedgerClient::new(&env);
    // get init balances
    let alice_init_balance = icp_ledger_client.icrc1_balance_of(alice_account());
    let swap_init_balance = icp_ledger_client.icrc1_balance_of(Account::from(env.swap_id));

    // bob buys the nft; first give to the canister the approval to spend icp_price
    assert!(icp_ledger_client
        .icrc2_approve(
            bob(),
            Account::from(env.swap_id),
            250_u64.into(),
            bob_account().subaccount
        )
        .is_ok());

    // buy
    assert!(swap_client
        .buy(bob(), my_nft.clone(), bob_account().subaccount)
        .is_err());

    // verify ownership
    assert_eq!(
        ledger_client.owner_of(my_nft.clone()).unwrap().unwrap(),
        alice()
    );
    // should no more be listed
    assert!(swap_client.get_listing(bob(), my_nft).unwrap().is_some());

    let alice_final_balance = icp_ledger_client.icrc1_balance_of(alice_account());
    let swap_final_balance = icp_ledger_client.icrc1_balance_of(Account::from(env.swap_id));
    assert_eq!(alice_init_balance, alice_final_balance);
    assert_eq!(swap_init_balance, swap_final_balance);
}
