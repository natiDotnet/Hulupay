//! Request body for PayBridge's `POST /api/v1/checkouts`, built from the
//! platform's `PaymentRequest`.
//!
//! PayBridge validates that `amount` (and every item `unitPrice`) is a JSON
//! number with at most 2 decimal places and that items sum exactly to the
//! total, so amounts are rendered from `Decimal` — never through f64.

use hulu_core::payment_gateway_error::PaymentGatewayError;
use hulu_core::payment_request::PaymentRequest;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayBridgeCreateCheckout {
    /// Merchant-side order id, echoed back in PayBridge webhooks as `reference`.
    pub reference: String,
    /// JSON number with at most 2 decimal places (PayBridge rejects more).
    pub amount: serde_json::Number,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PayBridgeItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<PayBridgeCustomer>,
    /// Browser redirect after a successful payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PayBridgeItem {
    pub name: String,
    pub quantity: i64,
    pub unit_price: serde_json::Number,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct PayBridgeCustomer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// Render a decimal as a JSON number with at most 2 decimal places
/// (PayBridge rejects more, and amounts must be JSON numbers, not strings).
/// Render a decimal as a JSON number with at most 2 decimal places
/// (PayBridge rejects more, and amounts must be JSON numbers, not strings).
fn decimal_to_number(value: rust_decimal::Decimal) -> Result<serde_json::Number, PaymentGatewayError> {
    let text = value.round_dp(2).normalize().to_string();
    serde_json::from_str::<serde_json::Number>(&text)
        .map_err(|_| PaymentGatewayError::InvalidResponse)
}

impl TryFrom<&PaymentRequest> for PayBridgeCreateCheckout {
    type Error = PaymentGatewayError;

    fn try_from(request: &PaymentRequest) -> Result<Self, PaymentGatewayError> {
        let amount = decimal_to_number(request.payment.amount)?;
        let items = request
            .items
            .iter()
            .map(|item| {
                Ok(PayBridgeItem {
                    name: item.name.clone(),
                    quantity: item.quantity as i64,
                    unit_price: decimal_to_number(item.price)?,
                })
            })
            .collect::<Result<Vec<_>, PaymentGatewayError>>()?;

        Ok(PayBridgeCreateCheckout {
            reference: request.payment.reference.clone(),
            amount,
            currency: request.payment.currency.clone(),
            items: if request.items.is_empty() {
                None
            } else {
                Some(items)
            },
            customer: Some(PayBridgeCustomer {
                name: Some(request.customer.name.clone()),
                email: Some(request.customer.email.clone()),
            }),
            return_url: Some(request.callbacks.success_url.clone()),
        })
    }
}
