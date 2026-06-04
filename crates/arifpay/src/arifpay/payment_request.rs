use crate::core::payment_request::PaymentRequest;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArifpayPaymentRequest {
    pub cancel_url: String,
    pub phone: String,
    pub email: String,
    pub nonce: String,
    pub success_url: String,
    pub error_url: String,
    pub notify_url: String,
    pub payment_methods: Vec<String>,
    pub expire_date: DateTime<Utc>,
    pub items: Vec<Item>,
    pub beneficiaries: Vec<Beneficiary>,
    pub currency: Option<String>,
    pub lang: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub name: String,
    pub quantity: u32,
    pub price: Decimal,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beneficiary {
    pub account_number: String,
    pub bank: String,
    pub amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtpRequest {
    pub session_id: String,
    pub phone: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyOtpRequest {
    pub session_id: String,
    pub otp: String,
}

impl From<PaymentRequest> for ArifpayPaymentRequest {
    fn from(value: PaymentRequest) -> Self {
        Self {
            phone: value.customer.phone,
            email: value.customer.email,
            items: value.items.into_iter().map(Into::into).collect(),
            currency: value.payment.currency,
            notify_url: value.callbacks.notify_url,
            success_url: value.callbacks.success_url,
            error_url: value.callbacks.error_url,
            cancel_url: value.callbacks.cancel_url,
            nonce: value.payment.reference.clone(),
            expire_date: value.payment.expire_date.unwrap_or_else(|| {
                Utc::now()
                    // .checked_add_days(Days::new(1))
                    .checked_add_signed(chrono::Duration::days(1))
                    .unwrap()
            }),
            payment_methods: value.payment.payment_methods,
            beneficiaries: value.beneficiaries.into_iter().map(Into::into).collect(),
            lang: value.payment.lang.unwrap_or_else(|| "ET".to_string()),
        }
    }
}
impl From<crate::core::payment_request::Beneficiary> for Beneficiary {
    fn from(value: crate::core::payment_request::Beneficiary) -> Self {
        Self {
            account_number: value.account_number,
            amount: value.amount,
            bank: value.bank,
        }
    }
}
impl From<crate::core::payment_request::Item> for Item {
    fn from(value: crate::core::payment_request::Item) -> Self {
        Self {
            name: value.name,
            description: value.description,
            quantity: value.quantity,
            price: value.price,
        }
    }
}
