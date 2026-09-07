use crate::domain::merchant_routing_rule::MerchantRoutingRule;
use crate::domain::routing_rule::{ConditionOperator, ConditionType};
use serde::{Deserialize, Serialize};
use toasty::Db;
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
    #[schema(value_type = String)]
    pub created_at: jiff::Timestamp,
    #[schema(value_type = String)]
    pub updated_at: jiff::Timestamp,
}

impl From<MerchantRoutingRule> for RuleResponse {
    fn from(m: MerchantRoutingRule) -> Self {
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
    db: Db,
}

impl ListRoutingRules {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
        page: u64,
        page_size: u64,
    ) -> anyhow::Result<PaginatedRulesResponse> {
        let mut db = self.db.clone();

        let all = MerchantRoutingRule::filter(
            MerchantRoutingRule::fields().merchant_id().eq(merchant_id),
        )
        .exec(&mut db)
        .await?;
        let total = all.len() as u64;

        let offset = if page > 1 { (page - 1) * page_size } else { 0 };
        let items = MerchantRoutingRule::filter(
            MerchantRoutingRule::fields().merchant_id().eq(merchant_id),
        )
        .order_by(MerchantRoutingRule::fields().priority().asc())
        .limit(page_size as usize)
        .offset(offset as usize)
        .exec(&mut db)
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
    db: Db,
}

impl CreateRoutingRule {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
        req: CreateRuleRequest,
    ) -> anyhow::Result<RuleResponse> {
        let mut db = self.db.clone();
        let now = crate::util::now_jiff();
        let row = toasty::create!(MerchantRoutingRule {
            merchant_id,
            priority: req.priority,
            enabled: req.enabled,
            condition_type: req.condition_type,
            operator: req.operator,
            condition_value: req.condition_value,
            target_provider_id: req.target_provider_id,
            fallback_provider_id: req.fallback_provider_id,
            created_at: now,
            updated_at: now,
        })
        .exec(&mut db)
        .await?;

        Ok(RuleResponse::from(row))
    }
}

#[derive(Clone)]
pub struct UpdateRoutingRule {
    db: Db,
}

impl UpdateRoutingRule {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        rule_id: Uuid,
        req: UpdateRuleRequest,
    ) -> anyhow::Result<RuleResponse> {
        let mut db = self.db.clone();

        let mut existing = MerchantRoutingRule::filter_by_id(rule_id)
            .first()
            .exec(&mut db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Rule not found"))?;

        // Apply conditional updates via a single toasty::update! per field.
        if let Some(v) = req.priority {
            toasty::update!(existing {
                priority: v,
                updated_at: crate::util::now_jiff()
            })
            .exec(&mut db)
            .await?;
        }
        if let Some(v) = req.enabled {
            toasty::update!(existing {
                enabled: v,
                updated_at: crate::util::now_jiff()
            })
            .exec(&mut db)
            .await?;
        }
        if let Some(v) = req.condition_type {
            toasty::update!(existing {
                condition_type: v,
                updated_at: crate::util::now_jiff()
            })
            .exec(&mut db)
            .await?;
        }
        if let Some(v) = req.operator {
            toasty::update!(existing {
                operator: v,
                updated_at: crate::util::now_jiff()
            })
            .exec(&mut db)
            .await?;
        }
        if let Some(v) = req.condition_value {
            toasty::update!(existing {
                condition_value: v,
                updated_at: crate::util::now_jiff()
            })
            .exec(&mut db)
            .await?;
        }
        if let Some(v) = req.target_provider_id {
            toasty::update!(existing {
                target_provider_id: v,
                updated_at: crate::util::now_jiff()
            })
            .exec(&mut db)
            .await?;
        }
        if let Some(v) = req.fallback_provider_id {
            toasty::update!(existing {
                fallback_provider_id: v,
                updated_at: crate::util::now_jiff()
            })
            .exec(&mut db)
            .await?;
        }

        // Reload the updated row to return the latest state.
        let row = MerchantRoutingRule::filter_by_id(rule_id)
            .first()
            .exec(&mut db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Rule not found"))?;

        Ok(RuleResponse::from(row))
    }
}

#[derive(Clone)]
pub struct DeleteRoutingRule {
    db: Db,
}

impl DeleteRoutingRule {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(&self, rule_id: Uuid) -> anyhow::Result<()> {
        let mut db = self.db.clone();
        MerchantRoutingRule::filter_by_id(rule_id)
            .delete()
            .exec(&mut db)
            .await?;
        Ok(())
    }
}
