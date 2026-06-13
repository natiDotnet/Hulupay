use std::sync::Arc;
use axum::extract::FromRef;
use crate::chapa::checkout;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use hulu_core::create_checkout::CreateCheckout;

pub fn chapa_routes<S>() -> OpenApiRouter<S>
where
    Arc<dyn CreateCheckout>: FromRef<S>,
    S: Clone + Send + Sync + 'static,
{
    OpenApiRouter::new().routes(routes!(checkout::chapa_checkout_handler,))
    // .with_state(checkout.clone())
}
