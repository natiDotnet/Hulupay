use hulu_core::payment_request::{
    Beneficiary, CallbackUrls, CustomerInfo, Item, PaymentOptions, PaymentRequest,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::application::helper::normalize;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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

    pub customization: Customization,

    pub meta: HashMap<String, serde_json::Value>,
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
                name: value.customization.title,
                description: value.customization.description,
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
