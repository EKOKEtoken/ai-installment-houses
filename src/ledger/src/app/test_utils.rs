use candid::Principal;
use dip721_rs::{TokenIdentifier, TokenMetadata};

use super::storage::{with_tokens_mut, TokensStorage};
use crate::utils::caller;

pub fn mock_token(id: u64) -> TokenMetadata {
    TokenMetadata {
        owner: Some(caller()),
        transferred_at: None,
        transferred_by: None,
        approved_at: None,
        approved_by: None,
        burned_at: None,
        burned_by: None,
        minted_at: 0,
        operator: None,
        is_burned: false,
        properties: vec![],
        token_identifier: TokenIdentifier::from(id),
        minted_by: Principal::anonymous(),
    }
}

pub fn with_mock_token<F>(id: u64, f: F) -> TokenMetadata
where
    F: FnOnce(&mut TokenMetadata),
{
    let mut token = mock_token(id);
    f(&mut token);
    token
}

pub fn store_mock_token(id: u64) -> TokenMetadata {
    let token = mock_token(id);

    with_tokens_mut(|tokens| {
        tokens.insert(TokenIdentifier::from(id).into(), token);
    });

    TokensStorage::get_token(&id.into()).unwrap()
}

pub fn store_mock_token_with<F>(id: u64, f: F) -> TokenMetadata
where
    F: FnOnce(&mut TokenMetadata),
{
    let token = with_mock_token(id, f);

    with_tokens_mut(|tokens| {
        tokens.insert(TokenIdentifier::from(id).into(), token);
    });

    TokensStorage::get_token(&id.into()).unwrap()
}

pub fn alice() -> Principal {
    Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap()
}

pub fn bob() -> Principal {
    Principal::from_text("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe").unwrap()
}

#[cfg(test)]
mod test {

    use crate::utils::time;

    use super::*;

    use dip721_rs::GenericValue;

    #[test]
    fn test_serialize_token_metadata() {
        let token = with_mock_token(65536u64, |token| {
            token.approved_at = Some(time());
            token.approved_by = Some(alice());
            token.burned_at = None;
            token.burned_by = None;
            token.minted_at = time();
            token.minted_by = alice();
            token.owner = Some(alice());
            token.transferred_at = Some(time());
            token.transferred_by = Some(alice());
            token.operator = Some(bob());
            token.is_burned = false;
            token.properties = vec![
                (
                    "floor".to_string(),
                    GenericValue::TextContent("12".to_string()),
                ),
                (
                    "rooms".to_string(),
                    GenericValue::TextContent("3".to_string()),
                ),
            ]
        });

        println!("{}", serde_json::to_string(&token).unwrap());
    }
}
