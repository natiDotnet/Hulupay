use crate::domain::error::DomainError;
use crate::domain::PaymentMethod;
use async_trait::async_trait;

pub struct InitializePayment {
    pub amount: f64,
    pub currency: String,
    pub method: PaymentMethod,
    pub reference: String,
    pub customer_phone: Option<String>,
    pub customer_email: Option<String>,
}

pub struct ProviderInitResponse {
    pub external_reference: String,
    pub checkout_url: String,
}

#[async_trait]
pub trait PaymentProvider: Send + Sync {
    fn name(&self) -> &'static str;
    async fn initialize_payment(&self, request: InitializePayment)
        -> Result<ProviderInitResponse, DomainError>;

    async fn verify_payment(&self, external_reference: String)
        -> Result<bool, DomainError>;
}
