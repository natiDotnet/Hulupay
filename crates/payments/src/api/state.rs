use crate::application::{
    CreatePaymentProvider, CreatePaymentProviderConfig, DeletePaymentProvider,
    DeletePaymentProviderConfig, GetPaymentProvider, GetPaymentProviderConfig,
    GetPaymentProviderConfigByProvider, HandleProviderWebhook, ListPaymentProviderConfigs,
    ListPaymentProviders, UpdatePaymentProvider, UpdatePaymentProviderConfig,
};
// use crate::application::initiate_payment::InitiatePayment;
use crate::ProviderEngine;
use hulu_core::create_checkout::CreateCheckout;
use std::sync::Arc;

#[derive(Clone)]
pub struct PaymentsState {
    pub provider_engine: ProviderEngine,
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

    pub handle_provider_webhook: HandleProviderWebhook,
    pub handle_create_checkout: Arc<dyn CreateCheckout>,
    // pub handle_initiate_payment: InitiatePayment,
}
