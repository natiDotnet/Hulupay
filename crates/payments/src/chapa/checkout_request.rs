use hulu_core::payment_request::{
    Beneficiary, CallbackUrls, CustomerInfo, Item, PaymentOptions, PaymentRequest,
};
use rust_decimal::Decimal;
use sea_orm::{ColIdx, Iden};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::application::helper::normalize;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ChapaInitializeRequest {
    pub amount: Decimal,
    pub currency: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub tx_ref: String,
    pub callback_url: String,
    pub return_url: String,
    pub customization: HashMap<String, serde_json::Value>,
    pub meta: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct ChapaInitializeRequestHelper {
    amount: Decimal,
    currency: String,
    email: String,
    first_name: String,
    last_name: String,
    phone_number: String,
    tx_ref: String,
    callback_url: String,
    return_url: String,

    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

impl<'de> Deserialize<'de> for ChapaInitializeRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = ChapaInitializeRequestHelper::deserialize(deserializer)?;

        let mut customization = HashMap::new();
        let mut meta = HashMap::new();

        for (key, value) in helper.extra {
            if let Some(field) = key
                .strip_prefix("customization[")
                .and_then(|s| s.strip_suffix(']'))
            {
                customization.insert(field.to_owned(), value);
            } else if let Some(field) = key.strip_prefix("meta[").and_then(|s| s.strip_suffix(']'))
            {
                meta.insert(field.to_owned(), value);
            }
        }

        Ok(Self {
            amount: helper.amount,
            currency: helper.currency,
            email: helper.email,
            first_name: helper.first_name,
            last_name: helper.last_name,
            phone_number: helper.phone_number,
            tx_ref: helper.tx_ref,
            callback_url: helper.callback_url,
            return_url: helper.return_url,
            customization,
            meta,
        })
    }
}

impl From<ChapaInitializeRequest> for PaymentRequest {
    fn from(value: ChapaInitializeRequest) -> Self {
        Self {
            customer: CustomerInfo {
                email: value.email,
                phone: normalize(&value.phone_number).unwrap(),
                name: format!("{} {}", value.first_name, value.last_name),
            },
            payment: PaymentOptions {
                amount: value.amount,
                currency: value.currency,
                reference: value.tx_ref,
                payment_methods: vec![],
                lang: None,
                expire_date: None,
            },
            items: vec![Item {
                image: None,
                quantity: 1,
                price: value.amount,
                name: value
                    .customization
                    .get("title")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                description: value
                    .customization
                    .get("description")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
            }],
            callbacks: CallbackUrls {
                notify_url: value.callback_url,
                success_url: value.return_url.clone(),
                cancel_url: value.return_url.clone(),
                error_url: value.return_url.clone(),
            },
            beneficiaries: vec![Beneficiary {
                account_number: "01320811436100".to_string(),
                bank: "AWINETAA".to_string(),
                amount: value.amount,
            }],
            metadata: value.meta,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Customization {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub invoices: Vec<Invoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub key: String,
    pub value: String,
}

pub struct Data {
    pub checkout_url: String,
}

pub struct ChapaInitializeResponse {
    pub message: String,
    pub status: String,
    pub data: Option<Data>,
}
