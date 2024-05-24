use candid::Nat;
use did::http::{HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::app::{App, Storage};

const MAX_LIMIT: usize = 20;

#[derive(Deserialize)]
struct TokenIdentifierReq {
    pub id: Nat,
}

#[derive(Deserialize)]
struct GetListingsReq {
    pub limit: usize,
    pub offset: usize,
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
            "get_listing" => Self::get_listing(req).await,
            "get_listings" => Self::get_listings(req).await,
            _ => HttpResponse::bad_request("unknown method".to_string()),
        }
    }

    async fn get_listing(req: HttpRequest) -> HttpResponse {
        let params = match req.decode_body::<TokenIdentifierReq>() {
            Ok(request) => request,
            Err(response) => return response,
        };
        App::get_listing(params.id.clone())
            .await
            .map(|listing| {
                let listing = listing.map(|listing| {
                    serde_json::json!({
                        "listing": {
                            "seller": listing.listing.seller,
                            "icpPrice": listing.listing.icp_price,
                            "expirationNs": listing.listing.expiration_ns,
                        },
                        "metadata": listing.metadata,
                    })
                });

                HttpResponse::ok(serde_json::json!({"listing": listing}))
            })
            .unwrap_or_else(|_| HttpResponse::not_found())
    }

    async fn get_listings(req: HttpRequest) -> HttpResponse {
        let params = match req.decode_body::<GetListingsReq>() {
            Ok(request) => request,
            Err(response) => return response,
        };
        let ids = Storage::listed_tokens();
        let total = ids.len();

        let ids_to_fetch = ids
            .iter()
            .skip(params.offset)
            .take(params.limit.min(MAX_LIMIT))
            .map(|id| id)
            .collect::<Vec<_>>();

        let mut listings = Vec::with_capacity(ids_to_fetch.len());
        for id in ids_to_fetch {
            if let Some(listing) = App::get_listing(id.clone()).await.unwrap() {
                listings.push(serde_json::json!(
                    {
                        "listing": {
                            "icpPrice": listing.listing.icp_price,
                            "seller": listing.listing.seller,
                            "expirationNs": listing.listing.expiration_ns,
                        },
                        "metadata": listing.metadata,
                    }
                ));
            }
        }

        let response = serde_json::json!({
            "listings": listings,
            "total": total,
        });

        HttpResponse::ok(response)
    }
}
