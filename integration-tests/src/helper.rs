use candid::{Nat, Principal};
use dip721_rs::TokenIdentifier;

use crate::actor::admin;
use crate::client::{LedgerClient, SwapClient};
use crate::TestEnv;

pub struct Helper;

pub const TEST_ICP_PRICE: u64 = 5_000_000u64;

impl Helper {
    pub fn mint_nft(env: &TestEnv, to: Principal) -> TokenIdentifier {
        let client = LedgerClient::new(env);
        let next_token_id = client.total_supply();

        assert!(client
            .mint(admin(), to, next_token_id.clone(), vec![])
            .is_ok());

        next_token_id
    }

    pub fn list_nft(env: &TestEnv, to: Principal) -> TokenIdentifier {
        let token_id = Self::mint_nft(env, to);
        let swap_client = SwapClient::new(env);

        // set swap as operator
        // set swap as operator of the nft
        let ledger_client = LedgerClient::new(env);
        assert!(ledger_client
            .approve(to, env.swap_id, token_id.clone())
            .is_ok());

        let icp_price = Nat::from(TEST_ICP_PRICE);
        let expiration_ns = Helper::time() * 2;
        assert!(swap_client
            .list(to, token_id.clone(), icp_price.clone(), expiration_ns, None)
            .is_ok());

        token_id
    }

    pub fn time() -> u64 {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        time.as_nanos() as u64
    }
}
