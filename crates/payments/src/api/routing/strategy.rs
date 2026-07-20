use crate::application::routing::routing_strategy::{
    GetRoutingStrategy, StrategyResponse, UpsertRoutingStrategy, UpsertStrategyRequest,
};
use axum::{Json, extract::{Path, State}, http::StatusCode};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[utoipa::path(
    get,
    tag = "routing",
    path = "/merchants/{merchant_id}/routing/strategy",
    security(("bearer_auth" = [])),
    params(
        ("merchant_id" = Uuid, Path, description = "Merchant ID")
    ),
    responses((status = OK, body = Option<StrategyResponse>))
)]
pub async fn get_strategy_handler(
    State(usecase): State<GetRoutingStrategy>,
    Path(merchant_id): Path<Uuid>,
) -> Result<Json<StrategyResponse>, StatusCode> {
    let result = usecase.execute(merchant_id).await.map_err(|e| {
        eprintln!("Error getting routing strategy: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    match result {
        Some(s) => Ok(Json(s)),
        None => Ok(Json(StrategyResponse {
            id: Uuid::nil(),
            merchant_id,
            strategy: crate::domain::routing_strategy::RoutingStrategy::Default,
            enabled: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })),
    }
}

#[utoipa::path(
    put,
    tag = "routing",
    path = "/merchants/{merchant_id}/routing/strategy",
    security(("bearer_auth" = [])),
    params(
        ("merchant_id" = Uuid, Path, description = "Merchant ID")
    ),
    request_body = UpsertStrategyRequest,
    responses((status = OK, body = StrategyResponse))
)]
pub async fn upsert_strategy_handler(
    State(usecase): State<UpsertRoutingStrategy>,
    Path(merchant_id): Path<Uuid>,
    Json(payload): Json<UpsertStrategyRequest>,
) -> Result<Json<StrategyResponse>, StatusCode> {
    let result = usecase.execute(merchant_id, payload).await.map_err(|e| {
        eprintln!("Error upserting routing strategy: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(result))
}
