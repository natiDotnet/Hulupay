use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, toasty::Model)]
pub struct PaymentProvider {
    #[key]
    #[auto]
    pub id: Uuid,
    #[unique]
    pub code: String,
    pub name: String,
    pub logo: String,
    pub is_active: bool,
    pub created_at: jiff::Timestamp,
}
