use crate::domain::routing_rule::{ConditionOperator, ConditionType};
use crate::domain::{MerchantRoutingRules, merchant_routing_rule};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// ── Request / Response DTOs ──────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CreateRuleRequest {
    pub priority: i32,
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub condition_type: ConditionType,
    pub operator: ConditionOperator,
    pub condition_value: String,
    pub target_provider_id: Uuid,
    pub fallback_provider_id: Option<Uuid>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateRuleRequest {
    pub priority: Option<i32>,
    pub enabled: Option<bool>,
    pub condition_type: Option<ConditionType>,
    pub operator: Option<ConditionOperator>,
    pub condition_value: Option<String>,
    pub target_provider_id: Option<Uuid>,
    pub fallback_provider_id: Option<Uuid>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct RuleResponse {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub priority: i32,
    pub enabled: bool,
    pub condition_type: ConditionType,
    pub operator: ConditionOperator,
    pub condition_value: String,
    pub target_provider_id: Uuid,
    pub fallback_provider_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<merchant_routing_rule::Model> for RuleResponse {
    fn from(m: merchant_routing_rule::Model) -> Self {
        Self {
            id: m.id,
            merchant_id: m.merchant_id,
            priority: m.priority,
            enabled: m.enabled,
            condition_type: m.condition_type,
            operator: m.operator,
            condition_value: m.condition_value,
            target_provider_id: m.target_provider_id,
            fallback_provider_id: m.fallback_provider_id,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PaginatedRulesResponse {
    pub items: Vec<RuleResponse>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

// ── Use cases ────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct ListRoutingRules {
    db: DatabaseConnection,
}

impl ListRoutingRules {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
        page: u64,
        page_size: u64,
    ) -> anyhow::Result<PaginatedRulesResponse> {
        let paginator = MerchantRoutingRules::find()
            .filter(merchant_routing_rule::Column::MerchantId.eq(merchant_id))
            .order_by_asc(merchant_routing_rule::Column::Priority)
            .paginate(&self.db, page_size);

        let total = paginator.num_items().await?;
        let items = paginator
            .fetch_page(page - 1)
            .await?
            .into_iter()
            .map(RuleResponse::from)
            .collect();

        Ok(PaginatedRulesResponse {
            items,
            total,
            page,
            page_size,
        })
    }
}

#[derive(Clone)]
pub struct CreateRoutingRule {
    db: DatabaseConnection,
}

impl CreateRoutingRule {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
        req: CreateRuleRequest,
    ) -> anyhow::Result<RuleResponse> {
        let row = merchant_routing_rule::ActiveModel {
            id: Set(Uuid::now_v7()),
            merchant_id: Set(merchant_id),
            priority: Set(req.priority),
            enabled: Set(req.enabled),
            condition_type: Set(req.condition_type),
            operator: Set(req.operator),
            condition_value: Set(req.condition_value),
            target_provider_id: Set(req.target_provider_id),
            fallback_provider_id: Set(req.fallback_provider_id),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
        }
        .insert(&self.db)
        .await?;

        Ok(RuleResponse::from(row))
    }
}

#[derive(Clone)]
pub struct UpdateRoutingRule {
    db: DatabaseConnection,
}

impl UpdateRoutingRule {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        rule_id: Uuid,
        req: UpdateRuleRequest,
    ) -> anyhow::Result<RuleResponse> {
        let existing = MerchantRoutingRules::find_by_id(rule_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Rule not found"))?;

        let mut active: merchant_routing_rule::ActiveModel = existing.into();
        if let Some(v) = req.priority {
            active.priority = Set(v);
        }
        if let Some(v) = req.enabled {
            active.enabled = Set(v);
        }
        if let Some(v) = req.condition_type {
            active.condition_type = Set(v);
        }
        if let Some(v) = req.operator {
            active.operator = Set(v);
        }
        if let Some(v) = req.condition_value {
            active.condition_value = Set(v);
        }
        if let Some(v) = req.target_provider_id {
            active.target_provider_id = Set(v);
        }
        if let Some(v) = req.fallback_provider_id {
            active.fallback_provider_id = Set(Some(v));
        }
        active.updated_at = Set(Utc::now());

        let row = active.update(&self.db).await?;
        Ok(RuleResponse::from(row))
    }
}

#[derive(Clone)]
pub struct DeleteRoutingRule {
    db: DatabaseConnection,
}

impl DeleteRoutingRule {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(&self, rule_id: Uuid) -> anyhow::Result<()> {
        MerchantRoutingRules::delete_by_id(rule_id)
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
