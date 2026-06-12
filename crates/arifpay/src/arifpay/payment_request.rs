use chrono::{DateTime, Utc};
use hulu_core::hulu_error::HuluError;
use hulu_core::payment_request;
use hulu_core::payment_request::{CallbackUrls, CustomerInfo, PaymentOptions, PaymentRequest};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub name: String,
    pub quantity: u32,
    pub price: Decimal,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
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

impl TryFrom<ArifpayPaymentRequest> for PaymentRequest {
    // fn from(value: ArifpayPaymentRequest) -> Self {}

    type Error = HuluError;

    fn try_from(value: ArifpayPaymentRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            payment: PaymentOptions {
                amount: value
                    .beneficiaries
                    .first()
                    .ok_or(HuluError::ResponseParseError)?
                    .amount,
                currency: value.currency.unwrap_or("ET".to_string()),
                reference: value.nonce,
                expire_date: Some(value.expire_date),
                lang: Some(value.lang),
                payment_methods: value.payment_methods,
            },
            customer: CustomerInfo {
                email: value.email,
                phone: value.phone,
                name: "".to_string(),
            },
            callbacks: CallbackUrls {
                notify_url: value.notify_url,
                cancel_url: value.cancel_url,
                success_url: value.success_url,
                error_url: value.error_url,
            },
            items: value.items.into_iter().map(Into::into).collect(),
            beneficiaries: value.beneficiaries.into_iter().map(Into::into).collect(),
            metadata: HashMap::new(),
        })
    }
}

impl From<&PaymentRequest> for ArifpayPaymentRequest {
    fn from(value: &PaymentRequest) -> Self {
        Self {
            phone: value.customer.phone.clone(),
            email: value.customer.email.clone(),
            items: value.items.clone().into_iter().map(Into::into).collect(),
            currency: Some(value.payment.currency.clone()),
            notify_url: value.callbacks.notify_url.clone(),
            success_url: value.callbacks.success_url.clone(),
            error_url: value.callbacks.error_url.clone(),
            cancel_url: value.callbacks.cancel_url.clone(),
            nonce: value.payment.reference.clone(),
            expire_date: value.payment.expire_date.unwrap_or_else(|| {
                Utc::now()
                    // .checked_add_days(Days::new(1))
                    .checked_add_signed(chrono::Duration::days(1))
                    .unwrap()
            }),
            payment_methods: value.payment.payment_methods.clone(),
            beneficiaries: value
                .beneficiaries
                .clone()
                .into_iter()
                .map(Into::into)
                .collect(),
            lang: value
                .payment
                .lang
                .clone()
                .unwrap_or_else(|| "ET".to_string()),
        }
    }
}
impl From<payment_request::Beneficiary> for Beneficiary {
    fn from(value: payment_request::Beneficiary) -> Self {
        Self {
            account_number: value.account_number,
            amount: value.amount,
            bank: value.bank,
        }
    }
}
impl From<payment_request::Item> for Item {
    fn from(value: payment_request::Item) -> Self {
        Self {
            name: value.name,
            description: value.description,
            quantity: value.quantity,
            price: value.price,
        }
    }
}

impl From<Item> for payment_request::Item {
    fn from(value: Item) -> Self {
        Self {
            name: value.name,
            description: value.description,
            quantity: value.quantity,
            price: value.price,
            image: None,
        }
    }
}

impl From<Beneficiary> for payment_request::Beneficiary {
    fn from(value: Beneficiary) -> Self {
        Self {
            account_number: value.account_number,
            amount: value.amount,
            bank: value.bank,
        }
    }
}
