mod state;
mod initialize;
mod verify;

pub use state::PaymentsState;

use axum::extract::FromRef;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::application::PaymentGateway;
use crate::infrastructure::ArifPayProvider;

impl FromRef<PaymentsState> for ArifPayProvider {
    fn from_ref(state: &PaymentsState) -> Self {
        state.arifpay_provider.clone()
    }
}

pub fn router(arifpay_provider: ArifPayProvider) -> OpenApiRouter {
    let state = PaymentsState {
        arifpay_provider,
    };

    OpenApiRouter::new()
        .routes(routes!(initialize::initialize_payment_handler))
        .routes(routes!(verify::verify_payment_handler))
        .with_state(state)
}
