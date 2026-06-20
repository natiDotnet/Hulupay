use crate::domain::error::DomainError;
use crate::domain::PaymentMethod;
use async_trait::async_trait;
use hulu_core::payment_method::GatewayProvider;
use sea_orm::sea_query::StringLen;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};
use utoipa::ToSchema;

// #[derive(Debug, Clone, PartialEq, sqlx::Type, serde::Serialize, serde::Deserialize)]
// #[sqlx(type_name = "provider", rename_all = "snake_case")]
#[derive(
    EnumIter,
    DeriveActiveEnum,
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
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "UPPERCASE"
)]
pub enum Provider {
    #[strum(serialize = "STRIPE", to_string = "STRIPE")]
    #[serde(rename = "STRIPE")]
    Stripe,

    #[strum(serialize = "HULU", to_string = "HULU")]
    #[serde(rename = "HULU")]
    Hulu,

    #[strum(serialize = "CHAPA", to_string = "CHAPA")]
    #[serde(rename = "CHAPA")]
    Chapa,

    #[strum(serialize = "ARIFPAY", to_string = "ARIFPAY")]
    #[serde(rename = "ARIFPAY")]
    ArifPay,
}

impl Provider {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Provider::ArifPay => "ARIFPAY",
            Provider::Chapa => "CHAPA",
            Provider::Stripe => "STRIPE",
            Provider::Hulu => "HULU",
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
            Self::Stripe => 300,   //  5 min
            Self::Chapa => 900,    // 15 min
            Self::ArifPay => 1200, // 20 min
            Self::Hulu => 1800,    // 30 min
        }
    }
}

impl From<GatewayProvider> for Provider {
    fn from(value: GatewayProvider) -> Self {
        match value {
            GatewayProvider::Arifpay => Provider::ArifPay,
            GatewayProvider::Chapa => Provider::Chapa,
            GatewayProvider::Hulu => Provider::Hulu,
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
