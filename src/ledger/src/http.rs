mod search;

use candid::Nat;
use did::http::{HttpRequest, HttpResponse};
use dip721_rs::Dip721;
use serde::Deserialize;

use self::search::{SearchParamsReq, SearchParamsSortBy};
use crate::app::storage::with_tokens;
use crate::app::App;

#[derive(Deserialize)]
struct TokenIdentifierReq {
    pub id: Nat,
}

pub struct HttpApi;

impl HttpApi {
    /// Handles an HTTP request
    pub async fn handle_http_request(req: HttpRequest) -> HttpResponse {
        // must be a GET request
        if req.method != "GET" {
            return HttpResponse::bad_request("expected GET method".to_string());
        }
        // Must be a JSON-RPC request
        if req.headers.get("content-type").map(|s| s.as_ref()) != Some("application/json") {
            return HttpResponse::bad_request(
                "expected content-type: application/json".to_string(),
            );
        }
        let method = match req.decode_method() {
            Ok(request) => request,
            Err(response) => return response,
        };

        match method.as_str() {
            "search" => Self::search(req),
            "dip721_metadata" => Self::dip721_metadata(),
            "dip721_name" => Self::dip721_name(),
            "dip721_symbol" => Self::dip721_symbol(),
            "dip721_logo" => Self::dip721_logo(),
            "dip721_total_unique_holders" => Self::dip721_total_unique_holders(),
            "dip721_token_metadata" => Self::dip721_token_metadata(req),
            "dip721_total_supply" => Self::dip721_total_supply(),
            _ => HttpResponse::bad_request("unknown method".to_string()),
        }
    }

    fn search(req: HttpRequest) -> HttpResponse {
        let params = match req.decode_body::<SearchParamsReq>() {
            Ok(request) => request,
            Err(response) => return response,
        };
        let tokens = with_tokens(|tokens| {
            let mut tokens = tokens
                .iter()
                .filter_map(|(_, metadata)| params.filter_tokens(metadata))
                .collect::<Vec<_>>();

            // sort
            if let Some(sort_by) = params.sort_by {
                match sort_by {
                    SearchParamsSortBy::Id => {
                        tokens.sort_by_key(|token| token.token_identifier.clone())
                    }
                    SearchParamsSortBy::MintedAt => tokens.sort_by_key(|token| token.minted_at),
                }
            }
            // limit and offset
            tokens
                .into_iter()
                .skip(params.offset)
                .take(params.limit)
                .collect::<Vec<_>>()
        });

        HttpResponse::ok(tokens)
    }

    fn dip721_metadata() -> HttpResponse {
        HttpResponse::ok(App::dip721_metadata())
    }

    fn dip721_name() -> HttpResponse {
        HttpResponse::ok(App::dip721_name())
    }

    fn dip721_symbol() -> HttpResponse {
        HttpResponse::ok(App::dip721_symbol())
    }

    fn dip721_logo() -> HttpResponse {
        HttpResponse::ok(App::dip721_logo())
    }

    fn dip721_total_unique_holders() -> HttpResponse {
        HttpResponse::ok(App::dip721_total_unique_holders())
    }

    fn dip721_token_metadata(req: HttpRequest) -> HttpResponse {
        let params = match req.decode_body::<TokenIdentifierReq>() {
            Ok(request) => request,
            Err(response) => return response,
        };
        App::dip721_token_metadata(params.id)
            .map(HttpResponse::ok)
            .unwrap_or_else(|_| HttpResponse::not_found())
    }

    fn dip721_total_supply() -> HttpResponse {
        HttpResponse::ok(App::dip721_total_supply())
    }
}
