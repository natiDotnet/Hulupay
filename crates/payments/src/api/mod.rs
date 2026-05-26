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
mod webhook;

pub use state::PaymentsState;
use std::collections::HashMap;

use axum::extract::FromRef;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::application::cache_service::CacheService;
use crate::application::initiate_payment::InitiatePayment;
use crate::application::{
    ArifWebhook, CreatePaymentProvider, CreatePaymentProviderConfig, DeletePaymentProvider,
    DeletePaymentProviderConfig, GetPaymentProvider, GetPaymentProviderConfig,
    GetPaymentProviderConfigByProvider, HandleProviderWebhook, ListPaymentProviderConfigs,
    ListPaymentProviders, PaymentProviderConfigRepository, PaymentProviderRepository,
    ProviderEngine, UpdatePaymentProvider, UpdatePaymentProviderConfig, WebhookHandler,
};
use crate::infrastructure::redis_service::RedisCacheService;
use auth::api::middleware::AuthRouterExt;
use auth::Role;
use deadpool_redis::{Config, Runtime};
use sea_orm::DatabaseConnection;
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

impl FromRef<PaymentsState> for HandleProviderWebhook {
    fn from_ref(state: &PaymentsState) -> Self {
        state.handle_provider_webhook.clone()
    }
}

pub fn router(pool: Pool<Postgres>, db: &DatabaseConnection) -> OpenApiRouter {
    let state = build_state(pool, db);

    let master_admin_routes = OpenApiRouter::new()
        .routes(routes!(
            create_payment_provider::create_payment_provider_handler,
            // list_payment_providers::list_payment_providers_handler,
        ))
        .routes(routes!(
            get_payment_provider::get_payment_provider_handler,
            update_payment_provider::update_payment_provider_handler,
            delete_payment_provider::delete_payment_provider_handler,
        ))
        .require_role(Role::MasterAdmin);

    let merchant_admin_routes = OpenApiRouter::new()
        .routes(routes!(
            create_payment_provider_config::create_payment_provider_config_handler,
            list_payment_provider_configs::list_payment_provider_configs_handler,
        ))
        .routes(routes!(
            get_payment_provider_config::get_payment_provider_config_handler,
            update_payment_provider_config::update_payment_provider_config_handler,
            delete_payment_provider_config::delete_payment_provider_config_handler,
        ))
        .routes(routes!(
        get_payment_provider_config_by_provider::get_payment_provider_config_by_provider_handler,
    ))
        .require_role(Role::MerchantAdmin);

    let authenticated_payment_routes = OpenApiRouter::new()
        .routes(routes!(
            list_payment_providers::list_payment_providers_handler,
        ))
        .routes(routes!(
            initialize::initialize_payment_handler,
            verify::verify_payment_handler,
        ))
        .require_auth();

    let public_routes = OpenApiRouter::new().routes(routes!(webhook::webhook_payment_handler));

    OpenApiRouter::new()
        .merge(master_admin_routes)
        .merge(merchant_admin_routes)
        .merge(authenticated_payment_routes)
        .merge(public_routes)
        .with_state(state)
}

fn build_state(pool: Pool<Postgres>, db: &DatabaseConnection) -> PaymentsState {
    // let provider_repo = build_provider_repository(pool.clone(), db);
    let config_repo = build_config_repository(pool.clone());
    // let transaction_repository = Arc::new(PgTransactionRepository::new(pool.clone()));
    let provider_engine = build_provider_engine(db);

    let mut webhook_handlers: HashMap<String, Arc<dyn WebhookHandler>> = HashMap::new();
    webhook_handlers.insert(
        "arifpay".to_string(),
        Arc::new(ArifWebhook::new(db.clone())),
    );
    let cache_service: Arc<dyn CacheService> =
        Arc::new(RedisCacheService::new(create_redis_pool()));

    PaymentsState {
        provider_engine: provider_engine.clone(),
        create_payment_provider: CreatePaymentProvider::new(db.clone()),
        get_payment_provider: GetPaymentProvider::new(db.clone()),
        update_payment_provider: UpdatePaymentProvider::new(db.clone()),
        delete_payment_provider: DeletePaymentProvider::new(db.clone()),
        list_payment_providers: ListPaymentProviders::new(db.clone()),
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
        handle_provider_webhook: HandleProviderWebhook::new(webhook_handlers),
        handle_initiate_payment: InitiatePayment::new(
            db.clone(),
            cache_service,
            provider_engine.clone(),
        ),
    }
}

fn build_provider_repository(
    pool: Pool<Postgres>,
    db: &DatabaseConnection,
) -> Arc<dyn PaymentProviderRepository> {
    // Arc::new(PgPaymentProviderRepository::new(pool, db.clone()))
}

fn build_config_repository(pool: Pool<Postgres>) -> Arc<dyn PaymentProviderConfigRepository> {
    // Arc::new(crate::infrastructure::PgPaymentProviderConfigRepository::new(pool))
}

fn build_provider_engine(
    // config_repo: &Arc<dyn PaymentProviderConfigRepository>,
    db: &DatabaseConnection,
    // transaction_repository: Arc<dyn TransactionRepository>,
) -> ProviderEngine {
    // let arifpay_config = ArifPayConfig::new(
    //     env::var("ARIFPAY_API_KEY").unwrap_or_else(|_| "test_key".to_string()),
    //     env::var("ARIFPAY_IS_TEST_KEY").unwrap_or_else(|_| "true".to_string()) == "true",
    // );
    // let arifpay_provider = crate::infrastructure::ArifPayProvider::new(arifpay_config);

    ProviderEngine::new(db.clone())
}

pub fn create_redis_pool() -> deadpool_redis::Pool {
    let cfg = Config::from_url("redis://127.0.0.1/");

    cfg.create_pool(Some(Runtime::Tokio1))
        .expect("Cannot create Redis pool")
}
