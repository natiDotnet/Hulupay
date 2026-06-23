use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use hulu_core::payment_method::GatewayProvider;
use hulu_core::request_context::RequestContext;
use std::collections::HashMap;
pub struct RequestCtx(pub RequestContext);
impl<S> FromRequestParts<S> for RequestCtx
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let header: HashMap<String, String> = parts
            .headers
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string().to_lowercase(),
                    v.to_str().unwrap_or("").to_string(),
                )
            })
            .collect();

        Ok(RequestCtx(RequestContext {
            merchant: "master".to_string(),
            provider: GatewayProvider::Hulu,
            use_provider: GatewayProvider::Hulu,
            headers: header,
        }))
    }
}
