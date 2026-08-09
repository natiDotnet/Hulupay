use crate::domain::payment_status::PaymentStatus;
use crate::domain::payment_method::PaymentMethod;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone, toasty::Model)]
pub struct MerchantWebhook {
    #[key]
    #[auto]
    pub id: Uuid,
    pub payment_order_id: Uuid,
    pub merchant_id: Uuid,
    pub status: PaymentStatus,
    pub provider_reference: String,
    pub payment_method: PaymentMethod,
    // #[column(type = "text")]
    pub amount: Decimal,
    // #[column(type = "text")]
    pub charge: Decimal,
    pub client_reference: String,
    pub txn_reference: String,
    pub created_at: jiff::Timestamp,
}
