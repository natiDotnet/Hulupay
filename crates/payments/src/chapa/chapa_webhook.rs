use chrono::{DateTime, Utc};
use hulu_core::{payment_gateway::PaymentStatus, payment_method::PaymentMethod};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Customization {
    pub title: Option<String>,
    pub description: Option<String>,
    pub logo: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChapaPaymentStatus {
    Pending,
    Success,
    Failed,
    Cancelled,
    Reversed,
    Refunding,
    Refunded,
}

impl From<PaymentStatus> for ChapaPaymentStatus {
    fn from(value: PaymentStatus) -> Self {
        match value {
            PaymentStatus::Success => Self::Success,
            PaymentStatus::Failed => Self::Failed,
            PaymentStatus::Pending => Self::Pending,
            PaymentStatus::Refunding => Self::Refunding,
            PaymentStatus::Refunded => Self::Refunded,
            PaymentStatus::Cancelled => Self::Cancelled,
            PaymentStatus::Reversed => Self::Reversed,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChapaWebhook {
    pub event: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub mobile: String,
    pub currency: String,
    pub amount: Decimal,
    pub charge: Decimal,
    pub status: ChapaPaymentStatus,
    pub mode: String,
    pub reference: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "type")]
    pub r#type: String,
    pub tx_ref: String,
    pub payment_method: ChapaPaymentMethod,
    pub customization: Customization,
    pub meta: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub enum ChapaPaymentMethod {
    #[serde(rename = "telebirr")]
    Telebirr,
    #[serde(rename = "mpasa")]
    Mpesa,
    #[serde(rename = "CBEBirr")]
    CbeBirr,
    #[serde(rename = "Coopay-Ebirr")]
    CoopayEbirr,
    #[serde(rename = "Enat Bank")]
    EnatBank,
    #[serde(rename = "awashbirr")]
    AwashBirr,
    #[serde(rename = "yaya")]
    Yaya,
    #[serde(rename = "boa_ussd")]
    BoaUssd,
    Unknown(String),
}

impl From<PaymentMethod> for ChapaPaymentMethod {
    fn from(value: PaymentMethod) -> Self {
        match value {
            PaymentMethod::Telebirr => Self::Telebirr,
            PaymentMethod::Mpesa => Self::Mpesa,
            PaymentMethod::CoopayEbirr => Self::CoopayEbirr,
            PaymentMethod::CbeBirr => Self::CbeBirr,
            PaymentMethod::AwashBirr => Self::AwashBirr,
            PaymentMethod::Yaya => Self::Yaya,
            PaymentMethod::Boa => Self::BoaUssd,
            _ => Self::Unknown(value.to_string()),
        }
    }
}
