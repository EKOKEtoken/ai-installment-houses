use std::collections::HashMap;

use dip721_rs::{GenericValue, TokenMetadata};
use serde::Deserialize;

const EXTRA_PROPS: [&str; 1] = [PROP_OWNER];
const PROP_OWNER: &str = "owner";

#[derive(Deserialize)]
pub struct SearchParamsReq {
    /// The maximum number of results to return
    pub limit: usize,
    /// The number of results to skip
    pub offset: usize,
    /// Search by property key whether it contains the given value
    pub properties: Option<HashMap<String, String>>,
    /// Sort the results by the given property key
    #[serde(rename = "sortBy")]
    pub sort_by: Option<SearchParamsSortBy>,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum SearchParamsSortBy {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "mintedAt")]
    MintedAt,
}

impl SearchParamsReq {
    pub fn filter_tokens(&self, token: TokenMetadata) -> Option<TokenMetadata> {
        if self.properties.is_none() {
            return Some(token);
        }
        let properties = self.properties.as_ref().unwrap();
        // check token owner
        if let Some(owner) = properties.get(PROP_OWNER) {
            println!(
                "owner: {:?}; {:?}",
                owner,
                token.owner.map(|owner| owner.to_string())
            );
            if token.owner.map(|owner| owner.to_string()).as_ref() != Some(owner) {
                return None;
            }
        }
        // check all the other properties
        if properties
            .iter()
            .filter(|(k, _)| !EXTRA_PROPS.contains(&k.as_str()))
            .all(|(key, value)| {
                token
                    .properties
                    .iter()
                    .find(|(k, _)| k == key)
                    .map(|(_, v)| Self::contains_generic_value(v, value))
                    .unwrap_or(false)
            })
        {
            Some(token)
        } else {
            None
        }
    }

    fn contains_generic_value(gv: &GenericValue, s: &str) -> bool {
        match gv {
            GenericValue::BlobContent(b) => b == s.as_bytes(),
            GenericValue::BoolContent(b) => s == "true" && *b || s == "false" && !*b,
            GenericValue::FloatContent(v) => v.to_string() == s,
            GenericValue::Int16Content(i) => i.to_string() == s,
            GenericValue::Int32Content(i) => i.to_string() == s,
            GenericValue::Int64Content(i) => i.to_string() == s,
            GenericValue::Int8Content(i) => i.to_string() == s,
            GenericValue::IntContent(i) => i.to_string() == s,
            GenericValue::Nat16Content(i) => i.to_string() == s,
            GenericValue::Nat32Content(i) => i.to_string() == s,
            GenericValue::Nat64Content(i) => i.to_string() == s,
            GenericValue::Nat8Content(i) => i.to_string() == s,
            GenericValue::NatContent(i) => i.to_string() == s,
            GenericValue::NestedContent(_) => false,
            GenericValue::Principal(p) => p.to_string() == s,
            GenericValue::TextContent(t) => t.to_lowercase().contains(&s.to_lowercase()),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::app::test_utils::{alice, bob, with_mock_token};

    #[test]
    fn test_should_decode_from_json() {
        let s = r#"
{
    "limit": 10,
    "offset": 5,
    "properties": {
        "owner": "zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae",
        "address": "Via Roma"
    },
    "sortBy": "id"
}
"#;

        let params: SearchParamsReq = serde_json::from_str(s).unwrap();
        assert_eq!(params.limit, 10);
        assert_eq!(params.offset, 5);
        assert_eq!(params.properties.unwrap().len(), 2);
        assert_eq!(params.sort_by.unwrap(), SearchParamsSortBy::Id);
    }

    #[test]
    fn test_should_filter_token_by_owner() {
        let token = with_mock_token(1, |token| {
            token.owner = Some(bob());
        });
        let params = SearchParamsReq {
            limit: 10,
            offset: 0,
            properties: Some({
                let mut map = HashMap::new();
                map.insert(PROP_OWNER.to_string(), bob().to_string());
                map
            }),
            sort_by: None,
        };

        assert_eq!(params.filter_tokens(token.clone()), Some(token));
    }

    #[test]
    fn test_should_filter_by_props() {
        let token = with_mock_token(1, |token| {
            token.owner = Some(bob());
            token.properties.push((
                "address".to_string(),
                GenericValue::TextContent("Via Roma 12".to_string()),
            ));
            token.properties.push((
                "city".to_string(),
                GenericValue::TextContent("Rome".to_string()),
            ));
            token
                .properties
                .push(("civic".to_string(), GenericValue::Int32Content(12)));
        });
        let params = SearchParamsReq {
            limit: 10,
            offset: 0,
            properties: Some({
                let mut map = HashMap::new();
                map.insert(PROP_OWNER.to_string(), bob().to_string());
                map.insert("address".to_string(), "via roma".to_string());
                map.insert("city".to_string(), "ro".to_string());
                map.insert("civic".to_string(), "12".to_string());

                map
            }),
            sort_by: None,
        };
        assert_eq!(params.filter_tokens(token.clone()), Some(token.clone()));

        let params = SearchParamsReq {
            limit: 10,
            offset: 0,
            properties: Some({
                let mut map = HashMap::new();
                map.insert(PROP_OWNER.to_string(), alice().to_string());
                map.insert("address".to_string(), "via roma".to_string());
                map.insert("city".to_string(), "ro".to_string());
                map.insert("civic".to_string(), "12".to_string());

                map
            }),
            sort_by: None,
        };
        assert!(params.filter_tokens(token.clone()).is_none());

        let params = SearchParamsReq {
            limit: 10,
            offset: 0,
            properties: Some({
                let mut map = HashMap::new();
                map.insert(PROP_OWNER.to_string(), bob().to_string());
                map.insert("address".to_string(), "via milan".to_string());
                map.insert("city".to_string(), "ro".to_string());
                map.insert("civic".to_string(), "12".to_string());

                map
            }),
            sort_by: None,
        };
        assert!(params.filter_tokens(token.clone()).is_none());
    }
}
