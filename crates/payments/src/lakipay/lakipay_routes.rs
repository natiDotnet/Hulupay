use crate::lakipay::checkout;
use axum::extract::FromRef;
use hulu_core::create_checkout::CreateCheckout;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn lakipay_routes<S>() -> OpenApiRouter<S>
where
    Arc<dyn CreateCheckout>: FromRef<S>,
    S: Clone + Send + Sync + 'static,
{
    OpenApiRouter::new().routes(routes!(checkout::laki_checkout_handler))
}
