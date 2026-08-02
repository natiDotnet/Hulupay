use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, toasty::Model)]
pub struct PaymentCustomer {
    #[key]
    #[auto]
    pub id: Uuid,
    pub payment_order_id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub account_number: Option<String>,
}
