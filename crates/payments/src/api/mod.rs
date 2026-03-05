mod state;
mod initialize;
mod verify;
mod create_payment_provider;
mod get_payment_provider;
mod update_payment_provider;
mod delete_payment_provider;
mod list_payment_providers;
mod create_payment_provider_config;
mod get_payment_provider_config;
mod update_payment_provider_config;
mod delete_payment_provider_config;
mod list_payment_provider_configs;

pub use state::PaymentsState;

use axum::extract::FromRef;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::application::{PaymentGateway, CreatePaymentProvider, GetPaymentProvider, UpdatePaymentProvider, DeletePaymentProvider, ListPaymentProviders, PaymentProviderRepository, CreatePaymentProviderConfig, GetPaymentProviderConfig, UpdatePaymentProviderConfig, DeletePaymentProviderConfig, ListPaymentProviderConfigs, PaymentProviderConfigRepository};
use crate::infrastructure::{ArifPayProvider, PgPaymentProviderRepository};
use sqlx::Pool;
use sqlx::Postgres;
use std::sync::Arc;
use auth::api::middleware::AuthRouterExt;
use auth::Role;

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

impl FromRef<PaymentsState> for CreatePaymentProviderConfig {
    fn from_ref(state: &PaymentsState) -> Self {
        state.create_payment_provider_config.clone()
    }
}

impl FromRef<PaymentsState> for GetPaymentProviderConfig {
    fn from_ref(state: &PaymentsState) -> Self {
        state.get_payment_provider_config.clone()
    }
}

impl FromRef<PaymentsState> for UpdatePaymentProviderConfig {
    fn from_ref(state: &PaymentsState) -> Self {
        state.update_payment_provider_config.clone()
    }
}

impl FromRef<PaymentsState> for DeletePaymentProviderConfig {
    fn from_ref(state: &PaymentsState) -> Self {
        state.delete_payment_provider_config.clone()
    }
}

impl FromRef<PaymentsState> for ListPaymentProviderConfigs {
    fn from_ref(state: &PaymentsState) -> Self {
        state.list_payment_provider_configs.clone()
    }
}

pub fn router(arifpay_provider: ArifPayProvider, pool: Pool<Postgres>) -> OpenApiRouter {
    let provider_repo: Arc<dyn PaymentProviderRepository> = Arc::new(PgPaymentProviderRepository::new(pool.clone()));
    let config_repo: Arc<dyn PaymentProviderConfigRepository> = Arc::new(crate::infrastructure::PgPaymentProviderConfigRepository::new(pool.clone()));
    
    let state = PaymentsState {
        arifpay_provider,
        create_payment_provider: CreatePaymentProvider::new(provider_repo.clone()),
        get_payment_provider: GetPaymentProvider::new(provider_repo.clone()),
        update_payment_provider: UpdatePaymentProvider::new(provider_repo.clone()),
        delete_payment_provider: DeletePaymentProvider::new(provider_repo.clone()),
        list_payment_providers: ListPaymentProviders::new(provider_repo.clone()),
        create_payment_provider_config: CreatePaymentProviderConfig::new(config_repo.clone(), provider_repo.clone()),
        get_payment_provider_config: GetPaymentProviderConfig::new(config_repo.clone()),
        update_payment_provider_config: UpdatePaymentProviderConfig::new(config_repo.clone()),
        delete_payment_provider_config: DeletePaymentProviderConfig::new(config_repo.clone()),
        list_payment_provider_configs: ListPaymentProviderConfigs::new(config_repo.clone()),
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
        .require_role(Role::MasterAdmin)
        .routes(
            routes!(
                create_payment_provider_config::create_payment_provider_config_handler,
                list_payment_provider_configs::list_payment_provider_configs_handler,
            )
        )
        .routes(
            routes!(
                get_payment_provider_config::get_payment_provider_config_handler,
                update_payment_provider_config::update_payment_provider_config_handler,
                delete_payment_provider_config::delete_payment_provider_config_handler,
            )
        )
        .require_role(Role::MerchantAdmin)
        .with_state(state)
}
