use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone, toasty::Model)]
pub struct PaymentItem {
    #[key]
    #[auto]
    pub id: Uuid,
    pub payment_order_id: Uuid,
    pub name: String,
    pub description: String,
    pub quantity: u32,
    pub image: Option<String>,
    // #[column(type = "text")]
    pub unit_price: Decimal,
    // #[column(type = "text")]
    pub total_price: Decimal,
}
