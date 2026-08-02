use crate::domain::payment_method::PaymentMethod;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, toasty::Model)]
pub struct ProviderPaymentMethod {
    #[key]
    #[auto]
    pub id: Uuid,
    pub provider_id: Uuid,
    #[index]
    pub payment_method_code: PaymentMethod,
    #[index]
    pub provider_method_code: String,
    // providers url segment
    pub provider_path_segment: Option<String>,
    pub is_active: bool,
}
