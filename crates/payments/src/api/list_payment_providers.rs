use super::create_payment_provider::ProviderResponse;
use crate::application::ListPaymentProviders;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: i64,

    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 {
    1
}
fn default_page_size() -> i64 {
    20
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedProvidersResponse {
    pub items: Vec<ProviderResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

#[utoipa::path(
    get,
    tag = "payment-providers",
    path = "/payment-providers",
    security(("bearer_auth" = [])),
    params(PaginationQuery),
    responses((status = OK, body = PaginatedProvidersResponse))
)]
pub async fn list_payment_providers_handler(
    State(usecase): State<ListPaymentProviders>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedProvidersResponse>, StatusCode> {
    let result = usecase
        .execute(pagination.page, pagination.page_size)
        .await
        .map_err(|e| {
            eprintln!("Error listing payment providers: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let items = result
        .items
        .iter()
        .map(|p| ProviderResponse {
            id: p.id,
            code: p.code.clone(),
            name: p.name.clone(),
            is_active: p.is_active,
            created_at: p.created_at,
        })
        .collect();

    Ok(Json(PaginatedProvidersResponse {
        items,
        total: result.total,
        page: result.page,
        page_size: result.page_size,
    }))
}
