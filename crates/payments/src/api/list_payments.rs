use crate::application::list_payments::PaymentItem;
use crate::application::ListPayments;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct MerchantIdParams {
    pub merchant_id: String,
}

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

#[derive(Serialize, ToSchema)]
pub struct PaginatedPaymentsResponse {
    pub items: Vec<PaymentItem>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

#[utoipa::path(
    get,
    tag = "payments",
    path = "/merchants/{merchant_id}/payments",
    security(("bearer_auth" = [])),
    params(MerchantIdParams, PaginationQuery),
    responses((status = OK, body = PaginatedPaymentsResponse))
)]
pub async fn list_payments_handler(
    State(usecase): State<ListPayments>,
    Path(params): Path<MerchantIdParams>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedPaymentsResponse>, StatusCode> {
    let merchant_id =
        uuid::Uuid::parse_str(&params.merchant_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let result = usecase
        .execute(merchant_id, pagination.page, pagination.page_size)
        .await
        .map_err(|e| {
            eprintln!("Error listing payments: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(PaginatedPaymentsResponse {
        items: result.items,
        total: result.total,
        page: result.page,
        page_size: result.page_size,
    }))
}
