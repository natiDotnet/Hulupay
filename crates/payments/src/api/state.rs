use crate::application::{CreatePaymentProvider, GetPaymentProvider, UpdatePaymentProvider, DeletePaymentProvider, ListPaymentProviders, CreatePaymentProviderConfig, GetPaymentProviderConfig, GetPaymentProviderConfigByProvider, UpdatePaymentProviderConfig, DeletePaymentProviderConfig, ListPaymentProviderConfigs};
use crate::infrastructure::ArifPayProvider;

#[derive(Clone)]
pub struct PaymentsState {
    pub arifpay_provider: ArifPayProvider,
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
}
