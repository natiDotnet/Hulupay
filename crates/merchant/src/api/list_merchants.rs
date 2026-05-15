use crate::application::{ListMerchants, PaginatedResponse};
use crate::MerchantResponse;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u64,

    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}
fn default_page_size() -> u64 {
    20
}

#[utoipa::path(
    get,
    tag = "merchant",
    path = "/merchants",
    security(("bearer_auth" = [])),
    params(PaginationQuery),
    responses((status = OK, body = PaginatedResponse<MerchantResponse>))
)]
pub async fn list_merchants_handler(
    State(usecase): State<ListMerchants>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<MerchantResponse>>, StatusCode> {
    let res = usecase
        .execute(pagination.page, pagination.page_size)
        .await
        .map_err(|e| {
            eprintln!("Error listing merchants: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(res))
}
