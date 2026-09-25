//! PayBridge checkout DTO (`GET/POST /api/v1/checkouts` responses). Amounts
//! arrive as decimal strings plus minor-unit integers; we keep both.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayBridgeCheckoutDto {
    pub checkout_id: String,
    /// Hosted payment page the customer completes the payment on.
    pub payment_url: String,
    pub reference: String,
    /// Decimal string, e.g. "500.00".
    pub amount: String,
    pub amount_minor: i64,
    pub currency: String,
    /// `created` | `pending` | `succeeded` | `failed` | `expired`.
    pub status: String,
    #[serde(default)]
    pub transaction_reference: Option<String>,
    #[serde(default)]
    pub paid_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
}
