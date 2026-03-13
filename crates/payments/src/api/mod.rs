mod create_payment_provider;
mod create_payment_provider_config;
mod delete_payment_provider;
mod delete_payment_provider_config;
mod get_payment_provider;
mod get_payment_provider_config;
mod get_payment_provider_config_by_provider;
mod initialize;
mod list_payment_provider_configs;
mod list_payment_providers;
mod state;
mod update_payment_provider;
mod update_payment_provider_config;
mod verify;

pub use state::PaymentsState;
use std::env;

use axum::extract::FromRef;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::application::{
    CreatePaymentProvider, CreatePaymentProviderConfig, DeletePaymentProvider,
    DeletePaymentProviderConfig, GetPaymentProvider, GetPaymentProviderConfig,
    GetPaymentProviderConfigByProvider, ListPaymentProviderConfigs, ListPaymentProviders,
    PaymentProviderConfigRepository, PaymentProviderRepository, ProviderEngine,
    UpdatePaymentProvider, UpdatePaymentProviderConfig,
};
use crate::infrastructure::PgPaymentProviderRepository;
use crate::ArifPayConfig;
use auth::api::middleware::AuthRouterExt;
use auth::Role;
use sqlx::Pool;
use sqlx::Postgres;
use std::sync::Arc;

impl FromRef<PaymentsState> for ProviderEngine {
    fn from_ref(state: &PaymentsState) -> Self {
        state.provider_engine.clone()
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

impl FromRef<PaymentsState> for GetPaymentProviderConfigByProvider {
    fn from_ref(state: &PaymentsState) -> Self {
        state.get_payment_provider_config_by_provider.clone()
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

pub fn router(pool: Pool<Postgres>) -> OpenApiRouter {
    let state = build_state(pool);

    OpenApiRouter::new()
        // Public payment endpoints (initialize and verify)
        .routes(routes!(
            initialize::initialize_payment_handler,
            verify::verify_payment_handler
        ))
        // MasterAdmin only endpoints - Payment Providers management
        .routes(
            routes!(
                create_payment_provider::create_payment_provider_handler,
                list_payment_providers::list_payment_providers_handler,
            )
        )
        .routes(
            routes!(
                get_payment_provider::get_payment_provider_handler,
                update_payment_provider::update_payment_provider_handler,
                delete_payment_provider::delete_payment_provider_handler,
            )
        )
        .require_role(Role::MasterAdmin)
        // MerchantAdmin only endpoints - Payment Provider Configs management
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
        .routes(
            routes!(
                get_payment_provider_config_by_provider::get_payment_provider_config_by_provider_handler,
            )
        )
        .require_role(Role::MerchantAdmin)
        .with_state(state)
}

fn build_state(pool: Pool<Postgres>) -> PaymentsState {
    let provider_repo = build_provider_repository(pool.clone());
    let config_repo = build_config_repository(pool.clone());
    let provider_engine = build_provider_engine();

    PaymentsState {
        provider_engine,
        create_payment_provider: CreatePaymentProvider::new(provider_repo.clone()),
        get_payment_provider: GetPaymentProvider::new(provider_repo.clone()),
        update_payment_provider: UpdatePaymentProvider::new(provider_repo.clone()),
        delete_payment_provider: DeletePaymentProvider::new(provider_repo.clone()),
        list_payment_providers: ListPaymentProviders::new(provider_repo.clone()),
        create_payment_provider_config: CreatePaymentProviderConfig::new(
            config_repo.clone(),
            provider_repo.clone(),
        ),
        get_payment_provider_config: GetPaymentProviderConfig::new(config_repo.clone()),
        get_payment_provider_config_by_provider: GetPaymentProviderConfigByProvider::new(
            config_repo.clone(),
            provider_repo.clone(),
        ),
        update_payment_provider_config: UpdatePaymentProviderConfig::new(config_repo.clone()),
        delete_payment_provider_config: DeletePaymentProviderConfig::new(config_repo.clone()),
        list_payment_provider_configs: ListPaymentProviderConfigs::new(config_repo.clone()),
    }
}

fn build_provider_repository(pool: Pool<Postgres>) -> Arc<dyn PaymentProviderRepository> {
    Arc::new(PgPaymentProviderRepository::new(pool))
}

fn build_config_repository(pool: Pool<Postgres>) -> Arc<dyn PaymentProviderConfigRepository> {
    Arc::new(crate::infrastructure::PgPaymentProviderConfigRepository::new(pool))
}

fn build_provider_engine() -> ProviderEngine {
    let arifpay_config = ArifPayConfig::new(
        env::var("ARIFPAY_API_KEY").unwrap_or_else(|_| "test_key".to_string()),
        env::var("ARIFPAY_IS_TEST_KEY").unwrap_or_else(|_| "true".to_string()) == "true",
    );
    let arifpay_provider = crate::infrastructure::ArifPayProvider::new(arifpay_config);

    let mut provider_engine = ProviderEngine::new();
    provider_engine.register_provider("arifpay", Arc::new(arifpay_provider));
    provider_engine
}
