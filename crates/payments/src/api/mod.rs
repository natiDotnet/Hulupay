pub mod arifpay_api;
mod create_payment_provider_config;
mod delete_payment_provider_config;
mod get_payment_provider_config;
mod get_payment_provider_config_by_provider;
mod initialize;
mod list_merchant_webhooks;
mod list_payment_provider_configs;
pub mod list_payments;
pub mod providers;
pub mod request_context;
pub mod routing;
mod state;
mod update_payment_provider_config;
mod verify;
mod webhook;

pub use state::PaymentsState;
use std::collections::HashMap;

use axum::extract::FromRef;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::application::cache_service::CacheService;
use crate::application::checkout::create_checkout::CreateCheckoutHandler;
use crate::application::checkout::payment_webhook::PaymentWebhookHandler;
use crate::application::checkout::verify_payment::VerifyPaymentHandler;
use crate::application::gateways::chapa::ChapaProvider;
use crate::application::routing::routing_rules::{
    CreateRoutingRule, DeleteRoutingRule, ListRoutingRules, UpdateRoutingRule,
};
use crate::application::routing::routing_strategy::{GetRoutingStrategy, UpsertRoutingStrategy};
use crate::application::{
    ArifWebhook, CreatePaymentProvider, CreatePaymentProviderConfig, DeletePaymentProvider,
    DeletePaymentProviderConfig, GetPaymentProvider, GetPaymentProviderConfig,
    GetPaymentProviderConfigByProvider, HandleProviderWebhook, ListMerchantWebhooks,
    ListPaymentProviderConfigs, ListPaymentProviders, ProviderEngine, RoutingEngine,
    UpdatePaymentProvider, UpdatePaymentProviderConfig, WebhookHandler, ListPayments,
};
use crate::arifpay::arifpay_service::ArifpayService;
use crate::chapa::chapa_service::ChapaService;
use crate::infrastructure::redis_service::RedisCacheService;
use crate::{ArifPayProvider, domain};
use auth::Role;
use auth::api::get_token_service;
use auth::api::middleware::AuthRouterExt;
use deadpool_redis::{Config, Runtime};
use domain::provider;
use hulu_core::create_checkout::{CreateCheckout, VerifyPayment};
use hulu_core::payment_gateway::PaymentGateway;
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

impl FromRef<PaymentsState> for ListMerchantWebhooks {
    fn from_ref(state: &PaymentsState) -> Self {
        state.list_merchant_webhooks.clone()
    }
}

impl FromRef<PaymentsState> for HandleProviderWebhook {
    fn from_ref(state: &PaymentsState) -> Self {
        state.handle_provider_webhook.clone()
    }
}

impl FromRef<PaymentsState> for Arc<dyn CreateCheckout> {
    fn from_ref(input: &PaymentsState) -> Self {
        input.handle_create_checkout.clone()
    }
}

impl FromRef<PaymentsState> for Arc<dyn VerifyPayment> {
    fn from_ref(input: &PaymentsState) -> Self {
        input.handle_verify_payment.clone()
    }
}

impl FromRef<PaymentsState> for PaymentWebhookHandler {
    fn from_ref(input: &PaymentsState) -> Self {
        input.handle_payment_webhook.clone()
    }
}
impl FromRef<PaymentsState> for toasty::Db {
    fn from_ref(input: &PaymentsState) -> Self {
        input.db.clone()
    }
}

impl FromRef<PaymentsState> for RoutingEngine {
    fn from_ref(state: &PaymentsState) -> Self {
        state.routing_engine.clone()
    }
}

impl FromRef<PaymentsState> for ListPayments {
    fn from_ref(state: &PaymentsState) -> Self {
        state.list_payments.clone()
    }
}

impl FromRef<PaymentsState> for GetRoutingStrategy {
    fn from_ref(state: &PaymentsState) -> Self {
        state.get_routing_strategy.clone()
    }
}

impl FromRef<PaymentsState> for UpsertRoutingStrategy {
    fn from_ref(state: &PaymentsState) -> Self {
        state.upsert_routing_strategy.clone()
    }
}

impl FromRef<PaymentsState> for ListRoutingRules {
    fn from_ref(state: &PaymentsState) -> Self {
        state.list_routing_rules.clone()
    }
}

impl FromRef<PaymentsState> for CreateRoutingRule {
    fn from_ref(state: &PaymentsState) -> Self {
        state.create_routing_rule.clone()
    }
}

impl FromRef<PaymentsState> for UpdateRoutingRule {
    fn from_ref(state: &PaymentsState) -> Self {
        state.update_routing_rule.clone()
    }
}

impl FromRef<PaymentsState> for DeleteRoutingRule {
    fn from_ref(state: &PaymentsState) -> Self {
        state.delete_routing_rule.clone()
    }
}
// impl FromRef<PaymentsState> for InitiatePayment {
//     fn from_ref(state: &PaymentsState) -> Self {
//         state.handle_initiate_payment.clone()
//     }
// }

pub fn router(db: &toasty::Db) -> OpenApiRouter {
    let state = build_state(db);

    let master_admin_routes = OpenApiRouter::new()
        .routes(routes!(
            providers::create_payment_provider::create_payment_provider_handler,
            // list_payment_providers::list_payment_providers_handler,
        ))
        .routes(routes!(
            providers::get_payment_provider::get_payment_provider_handler,
            providers::update_payment_provider::update_payment_provider_handler,
            providers::delete_payment_provider::delete_payment_provider_handler,
        ))
        .require_role(Role::MasterAdmin);

    let merchant_admin_routes = OpenApiRouter::new()
        .routes(routes!(
            create_payment_provider_config::create_payment_provider_config_handler,
            list_payment_provider_configs::list_payment_provider_configs_handler,
        ))
        .routes(routes!(
            list_merchant_webhooks::list_merchant_webhooks_handler,
        ))
        .routes(routes!(
            get_payment_provider_config::get_payment_provider_config_handler,
            update_payment_provider_config::update_payment_provider_config_handler,
            delete_payment_provider_config::delete_payment_provider_config_handler,
        ))
        .routes(routes!(
        get_payment_provider_config_by_provider::get_payment_provider_config_by_provider_handler,
    ))
        .routes(routes!(
            list_payments::list_payments_handler,
        ))
        .routes(routes!(
            routing::strategy::get_strategy_handler,
            routing::strategy::upsert_strategy_handler,
        ))
        .routes(routes!(
            routing::rules::list_rules_handler,
            routing::rules::create_rule_handler,
        ))
        .routes(routes!(
            routing::rules::update_rule_handler,
            routing::rules::delete_rule_handler,
        ))
        .require_role(Role::MerchantAdmin);

    let authenticated_payment_routes = OpenApiRouter::new()
        .routes(routes!(
            providers::list_payment_providers::list_payment_providers_handler,
        ))
        // .routes(routes!(
        //     create_checkout_session_handler,
        //     //     // initialize::initialize_payment_handler,
        //     //     verify::verify_payment_handler,
        // ))
        .require_auth();

    let public_routes = OpenApiRouter::new().routes(routes!(webhook::webhook_payment_handler));
    // .routes(routes!(
    //     arif::api::checkout::arifpay_checkout_handler,
    //     arif::chapa::checkout::chapa_checkout_handler,
    //     // create_checkout_session_handler,
    //     //     // initialize::initialize_payment_handler,
    //     //     verify::verify_payment_handler,
    // ))
    // .merge(arif::api::arifpay_route::arifpay_routes());
    // .merge(arif::chapa::chapa_routes(&state.handle_create_checkout));
    OpenApiRouter::<PaymentsState>::new()
        .nest(
            "/api",
            OpenApiRouter::new()
                .merge(master_admin_routes)
                .merge(merchant_admin_routes)
                .merge(authenticated_payment_routes)
                .merge(crate::chapa::chapa_routes::chapa_routes())
                .merge(crate::arifpay::arifpay_route::arifpay_routes())
                .merge(crate::lakipay::lakipay_routes::lakipay_routes())
                .layer(axum::middleware::from_fn(auth::api::authentication))
                .layer(axum::Extension(get_token_service().clone()))
                .layer(axum::Extension(db.clone()))
                .merge(public_routes),
        )
        // .merge(crate::chapa::chapa_routes::chapa_routes())
        // .merge(crate::arifpay::arifpay_route::arifpay_routes())
        .with_state(state)
}

fn build_state(db: &toasty::Db) -> PaymentsState {
    let provider_engine = build_provider_engine(db);

    let mut webhook_handlers: HashMap<String, Arc<dyn WebhookHandler>> = HashMap::new();
    webhook_handlers.insert(
        provider::Provider::ArifPay.to_string(),
        Arc::new(ArifWebhook::new(db.clone())),
    );
    let cache_service: Arc<dyn CacheService> =
        Arc::new(RedisCacheService::new(create_redis_pool()));
    let checkout_handler: Arc<dyn CreateCheckout> = Arc::new(CreateCheckoutHandler::new(
        db.clone(),
        cache_service.clone(),
        provider_engine.clone(),
        RoutingEngine::new(db.clone()),
    ));
    let verify_handler: Arc<dyn VerifyPayment> = Arc::new(VerifyPaymentHandler::new(
        db.clone(),
        cache_service.clone(),
        provider_engine.clone(),
    ));

    PaymentsState {
        db: db.clone(),
        provider_engine: provider_engine.clone(),
        routing_engine: RoutingEngine::new(db.clone()),
        create_payment_provider: CreatePaymentProvider::new(db.clone()),
        get_payment_provider: GetPaymentProvider::new(db.clone()),
        update_payment_provider: UpdatePaymentProvider::new(db.clone()),
        delete_payment_provider: DeletePaymentProvider::new(db.clone()),
        list_payment_providers: ListPaymentProviders::new(db.clone()),
        create_payment_provider_config: CreatePaymentProviderConfig::new(db.clone()),
        get_payment_provider_config: GetPaymentProviderConfig::new(db.clone()),
        get_payment_provider_config_by_provider: GetPaymentProviderConfigByProvider::new(
            db.clone(),
            provider_engine.clone(),
        ),
        update_payment_provider_config: UpdatePaymentProviderConfig::new(db.clone()),
        delete_payment_provider_config: DeletePaymentProviderConfig::new(db.clone()),
        list_payment_provider_configs: ListPaymentProviderConfigs::new(db.clone()),
        list_merchant_webhooks: ListMerchantWebhooks::new(db.clone()),
        list_payments: ListPayments::new(db.clone()),
        get_routing_strategy: GetRoutingStrategy::new(db.clone()),
        upsert_routing_strategy: UpsertRoutingStrategy::new(db.clone()),
        list_routing_rules: ListRoutingRules::new(db.clone()),
        create_routing_rule: CreateRoutingRule::new(db.clone()),
        update_routing_rule: UpdateRoutingRule::new(db.clone()),
        delete_routing_rule: DeleteRoutingRule::new(db.clone()),
        handle_provider_webhook: HandleProviderWebhook::new(webhook_handlers),
        handle_create_checkout: checkout_handler,
        handle_verify_payment: verify_handler,
        handle_payment_webhook: { PaymentWebhookHandler::new(db.clone(), provider_engine.clone()) },
        // handle_initiate_payment: InitiatePayment::new(
        //     db.clone(),
        //     cache_service,
        //     provider_engine.clone(),
        // ),
    }
}

// fn build_provider_repository(
//     pool: Pool<Postgres>,
//     db: &DatabaseConnection,
// ) -> Arc<dyn PaymentProviderRepository> {
//     // Arc::new(PgPaymentProviderRepository::new(pool, db.clone()))
// }

// fn build_config_repository(pool: Pool<Postgres>) -> Arc<dyn PaymentProviderConfigRepository> {
//     // Arc::new(crate::infrastructure::PgPaymentProviderConfigRepository::new(pool))
// }

fn build_provider_engine(db: &toasty::Db) -> ProviderEngine {
    // let arifpay_config = ArifPayConfig::new(
    //     env::var("ARIFPAY_API_KEY").unwrap_or_else(|_| "test_key".to_string()),
    //     env::var("ARIFPAY_IS_TEST_KEY").unwrap_or_else(|_| "true".to_string()) == "true",
    // );
    // let arifpay_provider = crate::infrastructure::ArifPayProvider::new(arifpay_config);
    let mut headers = reqwest::header::HeaderMap::new();
    // headers.insert("x-arifpay-key", config.api_key.parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    dbg!(&headers.values());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let mut providers: HashMap<String, Arc<dyn PaymentGateway>> = HashMap::new();
    providers.insert(
        provider::Provider::ArifPay.to_string(),
        Arc::new(ArifPayProvider::new(
            Arc::new(ArifpayService::new(client.clone())),
            db.clone(),
        )),
    );
    providers.insert(
        provider::Provider::Chapa.to_string(),
        Arc::new(ChapaProvider::new(
            Arc::new(ChapaService::new(client.clone())),
            db.clone(),
        )),
    );
    ProviderEngine::new(providers, db.clone())
}

pub fn create_redis_pool() -> deadpool_redis::Pool {
    let cfg = Config::from_url("redis://127.0.0.1:6379");

    let pool = cfg.create_pool(Some(Runtime::Tokio1)).map_err(|e| {
        tracing::error!("Redis pool creation failed: {:?}", e);
        e
    });

    pool.unwrap()
}
