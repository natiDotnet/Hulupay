use crate::domain::routing_strategy::RoutingStrategy;
use crate::domain::{MerchantRoutingStrategy, merchant_routing_strategy};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<merchant_routing_strategy::Model> for StrategyResponse {
    fn from(m: merchant_routing_strategy::Model) -> Self {
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
    db: DatabaseConnection,
}

impl GetRoutingStrategy {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(&self, merchant_id: Uuid) -> anyhow::Result<Option<StrategyResponse>> {
        let row = MerchantRoutingStrategy::find()
            .filter(merchant_routing_strategy::Column::MerchantId.eq(merchant_id))
            .one(&self.db)
            .await?;
        Ok(row.map(StrategyResponse::from))
    }
}

#[derive(Clone)]
pub struct UpsertRoutingStrategy {
    db: DatabaseConnection,
}

impl UpsertRoutingStrategy {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
        req: UpsertStrategyRequest,
    ) -> anyhow::Result<StrategyResponse> {
        let existing = MerchantRoutingStrategy::find()
            .filter(merchant_routing_strategy::Column::MerchantId.eq(merchant_id))
            .one(&self.db)
            .await?;

        let row = if let Some(existing) = existing {
            let mut active: merchant_routing_strategy::ActiveModel = existing.into();
            active.strategy = Set(req.strategy);
            active.enabled = Set(req.enabled);
            active.updated_at = Set(Utc::now());
            active.update(&self.db).await?
        } else {
            merchant_routing_strategy::ActiveModel {
                id: Set(Uuid::now_v7()),
                merchant_id: Set(merchant_id),
                strategy: Set(req.strategy),
                enabled: Set(req.enabled),
                created_at: Set(Utc::now()),
                updated_at: Set(Utc::now()),
            }
            .insert(&self.db)
            .await?
        };

        Ok(StrategyResponse::from(row))
    }
}
