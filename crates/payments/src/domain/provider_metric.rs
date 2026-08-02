use crate::domain::provider_status::ProviderStatus;
use uuid::Uuid;

/// Live health/metrics snapshot for a payment provider.
#[derive(Debug, Clone, toasty::Model)]
pub struct ProviderMetric {
    #[key]
    #[auto]
    pub id: Uuid,
    #[unique]
    #[index]
    pub provider_id: Uuid,
    pub current_status: ProviderStatus,
    /// Fraction in [0.0, 1.0].
    pub success_rate: f64,
    pub average_latency_ms: i64,
    pub error_rate: f64,
    pub last_health_check: Option<jiff::Timestamp>,
    pub updated_at: jiff::Timestamp,
}
