use crate::application::routing::routing_rules::{
    CreateRoutingRule, CreateRuleRequest, DeleteRoutingRule, ListRoutingRules, RuleResponse,
    UpdateRoutingRule, UpdateRuleRequest,
};
use axum::{Json, extract::{Path, Query, State}, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 { 1 }
fn default_page_size() -> u64 { 20 }

#[derive(Serialize, ToSchema)]
pub struct PaginatedRulesResponse {
    pub items: Vec<RuleResponse>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

#[utoipa::path(
    get,
    tag = "routing",
    path = "/merchants/{merchant_id}/routing/rules",
    security(("bearer_auth" = [])),
    params(
        ("merchant_id" = Uuid, Path, description = "Merchant ID"),
        PaginationQuery
    ),
    responses((status = OK, body = PaginatedRulesResponse))
)]
pub async fn list_rules_handler(
    State(usecase): State<ListRoutingRules>,
    Path(merchant_id): Path<Uuid>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedRulesResponse>, StatusCode> {
    let result = usecase.execute(merchant_id, pagination.page, pagination.page_size).await.map_err(|e| {
        eprintln!("Error listing routing rules: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(PaginatedRulesResponse {
        items: result.items,
        total: result.total,
        page: result.page,
        page_size: result.page_size,
    }))
}

#[utoipa::path(
    post,
    tag = "routing",
    path = "/merchants/{merchant_id}/routing/rules",
    security(("bearer_auth" = [])),
    params(
        ("merchant_id" = Uuid, Path, description = "Merchant ID")
    ),
    request_body = CreateRuleRequest,
    responses((status = CREATED, body = RuleResponse))
)]
pub async fn create_rule_handler(
    State(usecase): State<CreateRoutingRule>,
    Path(merchant_id): Path<Uuid>,
    Json(payload): Json<CreateRuleRequest>,
) -> Result<Json<RuleResponse>, StatusCode> {
    let result = usecase.execute(merchant_id, payload).await.map_err(|e| {
        eprintln!("Error creating routing rule: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(result))
}

#[utoipa::path(
    put,
    tag = "routing",
    path = "/merchants/{merchant_id}/routing/rules/{rule_id}",
    security(("bearer_auth" = [])),
    params(
        ("merchant_id" = Uuid, Path, description = "Merchant ID"),
        ("rule_id" = Uuid, Path, description = "Rule ID")
    ),
    request_body = UpdateRuleRequest,
    responses((status = OK, body = RuleResponse))
)]
pub async fn update_rule_handler(
    State(usecase): State<UpdateRoutingRule>,
    Path((merchant_id, rule_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateRuleRequest>,
) -> Result<Json<RuleResponse>, StatusCode> {
    let result = usecase.execute(rule_id, payload).await.map_err(|e| {
        eprintln!("Error updating routing rule: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    tag = "routing",
    path = "/merchants/{merchant_id}/routing/rules/{rule_id}",
    security(("bearer_auth" = [])),
    params(
        ("merchant_id" = Uuid, Path, description = "Merchant ID"),
        ("rule_id" = Uuid, Path, description = "Rule ID")
    ),
    responses((status = NO_CONTENT))
)]
pub async fn delete_rule_handler(
    State(usecase): State<DeleteRoutingRule>,
    Path((_merchant_id, rule_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    usecase.execute(rule_id).await.map_err(|e| {
        eprintln!("Error deleting routing rule: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(StatusCode::NO_CONTENT)
}
