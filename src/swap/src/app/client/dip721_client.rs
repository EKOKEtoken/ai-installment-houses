use candid::{Nat, Principal};
use dip721_rs::{NftError, TokenIdentifier, TokenMetadata};
use ic_cdk::api::call::CallResult;

/// Icrc ledger client
#[allow(dead_code)]
pub struct Dip721Client {
    principal: Principal,
}

impl From<Principal> for Dip721Client {
    fn from(principal: Principal) -> Self {
        Self::new(principal)
    }
}

impl Dip721Client {
    /// Create new icrc ledger client
    pub fn new(principal: Principal) -> Self {
        Self { principal }
    }

    #[allow(unused_variables)]
    pub async fn dip721_token_metadata(
        &self,
        token_identifier: &TokenIdentifier,
    ) -> CallResult<Result<TokenMetadata, NftError>> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(Ok(TokenMetadata {
                owner: Some(crate::utils::caller()),
                transferred_at: None,
                transferred_by: None,
                approved_at: None,
                approved_by: None,
                burned_at: None,
                burned_by: None,
                minted_at: 0,
                operator: Some(crate::utils::canister_id()),
                is_burned: false,
                properties: vec![],
                token_identifier: token_identifier.clone(),
                minted_by: Principal::anonymous(),
            }))
        }
        #[cfg(target_arch = "wasm32")]
        {
            let res: (Result<TokenMetadata, NftError>,) =
                ic_cdk::call(self.principal, "dip721_token_metadata", (token_identifier,)).await?;

            Ok(res.0)
        }
    }

    #[allow(unused_variables)]
    pub async fn dip721_transfer_from(
        &self,
        from: Principal,
        to: Principal,
        token_identifier: &TokenIdentifier,
    ) -> CallResult<Result<Nat, NftError>> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(Ok(Nat::from(1u64)))
        }
        #[cfg(target_arch = "wasm32")]
        {
            let res: (Result<Nat, NftError>,) = ic_cdk::call(
                self.principal,
                "dip721_transfer_from",
                (token_identifier, from, to),
            )
            .await?;

            Ok(res.0)
        }
    }
}
