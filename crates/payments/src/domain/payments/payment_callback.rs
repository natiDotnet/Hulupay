use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, toasty::Model)]
pub struct PaymentCallback {
    #[key]
    #[auto]
    pub id: Uuid,
    pub payment_order_id: Uuid,
    pub success_url: String,
    pub error_url: String,
    pub cancel_url: String,
    pub notify_url: String,
}
