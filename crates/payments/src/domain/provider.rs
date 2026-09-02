use crate::domain::PaymentMethod;
use crate::domain::error::DomainError;
use async_trait::async_trait;
use hulu_core::payment_method::GatewayProvider;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};
use utoipa::ToSchema;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    ToSchema,
    Display,
    AsRefStr,
    EnumString,
    toasty::Embed,
)]
#[column(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum Provider {
    Stripe,
    Hulu,
    Chapa,
    ArifPay,
    LakiPay,
    StarPay,
    Simulator,
}

impl Provider {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Provider::ArifPay => "ARIFPAY",
            Provider::Chapa => "CHAPA",
            Provider::Stripe => "STRIPE",
            Provider::Hulu => "HULU",
            Provider::Simulator => "SIMULATOR",
            Provider::LakiPay => "LAKIPAY",
            Provider::StarPay => "STARPAY"
        }
    }
}

impl Provider {
    pub fn max_retries(&self) -> i32 {
        3
    }

    // SLA window in seconds before the watchdog kicks in
    pub fn sla_timeout_secs(&self) -> i64 {
        match self {
            Self::Stripe => 300,    //  5 min
            Self::Chapa => 900,     // 15 min
            Self::ArifPay => 1200,  // 20 min
            Self::Hulu => 1800,     // 30 min
            Self::Simulator => 300, //  5 min (instant for testing)
            Self::LakiPay => 300,
            Self::StarPay => 300
        }
    }
}

impl From<GatewayProvider> for Provider {
    fn from(value: GatewayProvider) -> Self {
        match value {
            GatewayProvider::ArifPay => Provider::ArifPay,
            GatewayProvider::Chapa => Provider::Chapa,
            GatewayProvider::Hulu => Provider::Hulu,
            GatewayProvider::Simulator => Provider::Simulator,
            GatewayProvider::LakiPay => Provider::LakiPay,
            GatewayProvider::StarPay => Provider::StarPay
        }
    }
}

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
    async fn initialize_payment(
        &self,
        request: InitializePayment,
    ) -> Result<ProviderInitResponse, DomainError>;

    async fn verify_payment(&self, external_reference: String) -> Result<bool, DomainError>;
}
