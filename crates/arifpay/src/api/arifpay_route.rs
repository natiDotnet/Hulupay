use crate::api::{checkout, verify};
use axum::extract::FromRef;
use hulu_core::create_checkout::{CreateCheckout, VerifyPayment};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn arifpay_routes<S>() -> OpenApiRouter<S>
where
    Arc<dyn CreateCheckout>: FromRef<S>,
    Arc<dyn VerifyPayment>: FromRef<S>,
    S: Clone + Send + Sync + 'static,
{
    OpenApiRouter::new().routes(routes!(
        checkout::arifpay_checkout_handler,
        verify::arifpay_verify_handler
    ))
    // .with_state(checkout.clone())
}
