use crate::domain::routing_strategy::RoutingStrategy;
use uuid::Uuid;

/// One row per merchant: the active high-level routing strategy.
#[derive(Debug, Clone, toasty::Model)]
pub struct MerchantRoutingStrategy {
    #[key]
    #[auto]
    pub id: Uuid,
    #[unique]
    #[index]
    pub merchant_id: Uuid,
    pub strategy: RoutingStrategy,
    pub enabled: bool,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
}
