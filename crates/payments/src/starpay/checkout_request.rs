use std::collections::HashMap;
use jiff::{Timestamp, ToSpan};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use hulu_core::payment_request::PaymentRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub order_reference: String,
    pub custom_field: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StarItem {
    pub product_id: String,
    pub quantity: u32,
    pub item_name: String,
    pub unit_price: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct StarCheckoutRequest {
    pub amount: Decimal,
    pub description: String,
    pub currency: String,
    pub customer_name: String,
    pub customer_phone_number: String,
    pub items: Vec<StarItem>,
    #[serde(rename = "callbackURL")]
    pub callback_url: String,
    pub customer_email: String,
    pub expired_at: jiff::Timestamp,
    pub redirect_url: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl From<&PaymentRequest> for StarCheckoutRequest {
    fn from(request: &PaymentRequest) -> Self {
        Self {
            amount: request.payment.amount,
            description: request.items.first().unwrap().description.clone(),
            currency: request.payment.currency.clone(),
            customer_name: request.customer.name.clone(),
            customer_phone_number: request.customer.phone.clone(),
            customer_email: request.customer.email.clone(),
            items: request.items.iter().map(|t| {
                StarItem {
                    product_id: t.name.clone(),
                    item_name: t.name.clone(),
                    quantity: t.quantity,
                    unit_price: t.price,
                }
            }).collect(),
            callback_url: request.callbacks.notify_url.clone(),
            expired_at: request.payment.expire_date.unwrap_or_else(|| Timestamp::now().checked_add(1.days()).unwrap()),
            redirect_url: request.callbacks.success_url.clone(),
            metadata: request.metadata.clone(),
        }
    }
}