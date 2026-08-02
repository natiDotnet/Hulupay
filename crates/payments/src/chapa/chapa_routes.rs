use crate::chapa::{checkout, verify};
use axum::extract::FromRef;
use hulu_core::create_checkout::{CreateCheckout, VerifyPayment};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn chapa_routes<S>() -> OpenApiRouter<S>
where
    Arc<dyn CreateCheckout>: FromRef<S>,
    Arc<dyn VerifyPayment>: FromRef<S>,
    toasty::Db: FromRef<S>,
    S: Clone + Send + Sync + 'static,
{
    OpenApiRouter::new().routes(routes!(
        checkout::chapa_checkout_handler,
        verify::chapa_verify_handler
    ))
}
