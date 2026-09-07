use crate::application::checkout::payment_webhook::PaymentWebhookHandler;
use crate::application::routing::routing_rules::{
    CreateRoutingRule, DeleteRoutingRule, ListRoutingRules, UpdateRoutingRule,
};
use crate::application::routing::routing_strategy::{GetRoutingStrategy, UpsertRoutingStrategy};
use crate::application::{
    CreatePaymentProvider, CreatePaymentProviderConfig, DeletePaymentProvider,
    DeletePaymentProviderConfig, GetPaymentProvider, GetPaymentProviderConfig,
    GetPaymentProviderConfigByProvider, HandleProviderWebhook, ListMerchantWebhooks,
    ListPaymentProviderConfigs, ListPaymentProviders, ProviderEngine, RoutingEngine,
    UpdatePaymentProvider, UpdatePaymentProviderConfig, ListPayments,
};
use hulu_core::create_checkout::{CreateCheckout, VerifyPayment};
use std::sync::Arc;

#[derive(Clone)]
pub struct PaymentsState {
    pub db: toasty::Db,
    pub provider_engine: ProviderEngine,
    pub routing_engine: RoutingEngine,
    pub create_payment_provider: CreatePaymentProvider,
    pub get_payment_provider: GetPaymentProvider,
    pub update_payment_provider: UpdatePaymentProvider,
    pub delete_payment_provider: DeletePaymentProvider,
    pub list_payment_providers: ListPaymentProviders,
    pub create_payment_provider_config: CreatePaymentProviderConfig,
    pub get_payment_provider_config: GetPaymentProviderConfig,
    pub get_payment_provider_config_by_provider: GetPaymentProviderConfigByProvider,
    pub update_payment_provider_config: UpdatePaymentProviderConfig,
    pub delete_payment_provider_config: DeletePaymentProviderConfig,
    pub list_payment_provider_configs: ListPaymentProviderConfigs,
    pub list_merchant_webhooks: ListMerchantWebhooks,
    pub get_routing_strategy: GetRoutingStrategy,
    pub upsert_routing_strategy: UpsertRoutingStrategy,
    pub list_routing_rules: ListRoutingRules,
    pub create_routing_rule: CreateRoutingRule,
    pub update_routing_rule: UpdateRoutingRule,
    pub delete_routing_rule: DeleteRoutingRule,
    pub list_payments: ListPayments,

    pub handle_provider_webhook: HandleProviderWebhook,
    pub handle_create_checkout: Arc<dyn CreateCheckout>,
    pub handle_verify_payment: Arc<dyn VerifyPayment>,
    pub handle_payment_webhook: PaymentWebhookHandler,
}
