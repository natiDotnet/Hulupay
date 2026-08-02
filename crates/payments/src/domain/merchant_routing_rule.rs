use crate::domain::routing_rule::{ConditionOperator, ConditionType};
use uuid::Uuid;

/// A merchant's explicit, ordered routing rule. Lower `priority` runs first.
#[derive(Debug, Clone, toasty::Model)]
pub struct MerchantRoutingRule {
    #[key]
    #[auto]
    pub id: Uuid,
    #[index]
    pub merchant_id: Uuid,
    /// Lower number = higher priority (runs first).
    pub priority: i32,
    pub enabled: bool,
    pub condition_type: ConditionType,
    pub operator: ConditionOperator,
    /// Free-form value compared against the condition (e.g. "TELEBIRR", "ETB", "5000").
    pub condition_value: String,
    /// Provider to route to when the rule matches.
    pub target_provider_id: Uuid,
    /// Optional fallback provider if the target is unavailable.
    pub fallback_provider_id: Option<Uuid>,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
}
