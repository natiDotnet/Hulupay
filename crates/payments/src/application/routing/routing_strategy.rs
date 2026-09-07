use crate::domain::merchant_routing_strategy::MerchantRoutingStrategy;
use crate::domain::routing_strategy::RoutingStrategy;
use serde::{Deserialize, Serialize};
use toasty::Db;
use utoipa::ToSchema;
use uuid::Uuid;

// ── Request / Response DTOs ──────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct UpsertStrategyRequest {
    pub strategy: RoutingStrategy,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Serialize, ToSchema)]
pub struct StrategyResponse {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub strategy: RoutingStrategy,
    pub enabled: bool,
    #[schema(value_type = String)]
    pub created_at: jiff::Timestamp,
    #[schema(value_type = String)]
    pub updated_at: jiff::Timestamp,
}

impl From<MerchantRoutingStrategy> for StrategyResponse {
    fn from(m: MerchantRoutingStrategy) -> Self {
        Self {
            id: m.id,
            merchant_id: m.merchant_id,
            strategy: m.strategy,
            enabled: m.enabled,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

// ── Use case ─────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct GetRoutingStrategy {
    db: Db,
}

impl GetRoutingStrategy {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(&self, merchant_id: Uuid) -> anyhow::Result<Option<StrategyResponse>> {
        let mut db = self.db.clone();
        let row = MerchantRoutingStrategy::filter(
            MerchantRoutingStrategy::fields()
                .merchant_id()
                .eq(merchant_id),
        )
        .first()
        .exec(&mut db)
        .await?;
        Ok(row.map(StrategyResponse::from))
    }
}

#[derive(Clone)]
pub struct UpsertRoutingStrategy {
    db: Db,
}

impl UpsertRoutingStrategy {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
        req: UpsertStrategyRequest,
    ) -> anyhow::Result<StrategyResponse> {
        let mut db = self.db.clone();

        let existing = MerchantRoutingStrategy::filter(
            MerchantRoutingStrategy::fields()
                .merchant_id()
                .eq(merchant_id),
        )
        .first()
        .exec(&mut db)
        .await?;

        if let Some(mut existing) = existing {
            toasty::update!(existing {
                strategy: req.strategy,
                enabled: req.enabled,
                updated_at: crate::util::now_jiff(),
            })
            .exec(&mut db)
            .await?;
            Ok(StrategyResponse::from(existing))
        } else {
            let now = crate::util::now_jiff();
            let row = toasty::create!(MerchantRoutingStrategy {
                merchant_id,
                strategy: req.strategy,
                enabled: req.enabled,
                created_at: now,
                updated_at: now,
            })
            .exec(&mut db)
            .await?;
            Ok(StrategyResponse::from(row))
        }
    }
}
