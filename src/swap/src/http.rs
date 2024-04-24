use candid::Nat;
use did::http::{HttpRequest, HttpResponse};
use serde::Deserialize;

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
            "get_listing" => Self::get_listing(req).await,
            _ => HttpResponse::bad_request("unknown method".to_string()),
        }
    }

    async fn get_listing(req: HttpRequest) -> HttpResponse {
        let params = match req.decode_body::<TokenIdentifierReq>() {
            Ok(request) => request,
            Err(response) => return response,
        };
        App::get_listing(params.id)
            .await
            .map(HttpResponse::ok)
            .unwrap_or_else(|_| HttpResponse::not_found())
    }
}
