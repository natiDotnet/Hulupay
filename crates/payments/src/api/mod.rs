mod state;
mod initialize;
mod verify;
mod create_payment_provider;
mod get_payment_provider;
mod update_payment_provider;
mod delete_payment_provider;
mod list_payment_providers;

pub use state::PaymentsState;

use axum::extract::FromRef;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::application::{PaymentGateway, CreatePaymentProvider, GetPaymentProvider, UpdatePaymentProvider, DeletePaymentProvider, ListPaymentProviders, PaymentProviderRepository};
use crate::infrastructure::{ArifPayProvider, PgPaymentProviderRepository};
use sqlx::Pool;
use sqlx::Postgres;
use std::sync::Arc;

impl FromRef<PaymentsState> for ArifPayProvider {
    fn from_ref(state: &PaymentsState) -> Self {
        state.arifpay_provider.clone()
    }
}

impl FromRef<PaymentsState> for CreatePaymentProvider {
    fn from_ref(state: &PaymentsState) -> Self {
        state.create_payment_provider.clone()
    }
}

impl FromRef<PaymentsState> for GetPaymentProvider {
    fn from_ref(state: &PaymentsState) -> Self {
        state.get_payment_provider.clone()
    }
}

impl FromRef<PaymentsState> for UpdatePaymentProvider {
    fn from_ref(state: &PaymentsState) -> Self {
        state.update_payment_provider.clone()
    }
}

impl FromRef<PaymentsState> for DeletePaymentProvider {
    fn from_ref(state: &PaymentsState) -> Self {
        state.delete_payment_provider.clone()
    }
}

impl FromRef<PaymentsState> for ListPaymentProviders {
    fn from_ref(state: &PaymentsState) -> Self {
        state.list_payment_providers.clone()
    }
}

pub fn router(arifpay_provider: ArifPayProvider, pool: Pool<Postgres>) -> OpenApiRouter {
    let provider_repo: Arc<dyn PaymentProviderRepository> = Arc::new(PgPaymentProviderRepository::new(pool.clone()));
    
    let state = PaymentsState {
        arifpay_provider,
        create_payment_provider: CreatePaymentProvider::new(provider_repo.clone()),
        get_payment_provider: GetPaymentProvider::new(provider_repo.clone()),
        update_payment_provider: UpdatePaymentProvider::new(provider_repo.clone()),
        delete_payment_provider: DeletePaymentProvider::new(provider_repo.clone()),
        list_payment_providers: ListPaymentProviders::new(provider_repo.clone()),
    };

    OpenApiRouter::new()
        .routes(routes!(initialize::initialize_payment_handler))
        .routes(routes!(verify::verify_payment_handler))
        .routes(
            routes!(
                create_payment_provider::create_payment_provider_handler,
                list_payment_providers::list_payment_providers_handler,))
        .routes(
            routes!(
            get_payment_provider::get_payment_provider_handler,
            update_payment_provider::update_payment_provider_handler,
            delete_payment_provider::delete_payment_provider_handler,
        ))
        .with_state(state)
}
