use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Success/failure body shaped exactly like the upstream LakiPay v2
/// checkout response (lakipay-validation/report.md §20).
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct LakiCheckoutResponse {
    pub success: bool,
    pub status: Option<String>,
    pub message: String,
    pub payment_url: Option<String>,
    pub reference_id: Option<String>,
    pub lakipay_transaction_id: Option<String>,
    pub merchant_pays_fee: Option<bool>,
}

impl LakiCheckoutResponse {
    /// Build the success body from the core checkout result. Mirrors the
    /// upstream field set: `payment_url` carries the hosted-checkout URL,
    /// `reference_id` echoes the merchant reference, and
    /// `lakipay_transaction_id` stays null because HuluPay's core response
    /// does not surface the provider-side transaction id.
    pub fn success(payment_url: String, reference_id: String) -> Self {
        Self {
            success: true,
            status: Some("PENDING".to_string()),
            message: "Hosted checkout created successfully".to_string(),
            payment_url: Some(payment_url),
            reference_id: Some(reference_id),
            lakipay_transaction_id: None,
            merchant_pays_fee: Some(false),
        }
    }
}
