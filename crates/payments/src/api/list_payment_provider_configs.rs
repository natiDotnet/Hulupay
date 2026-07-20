use super::create_payment_provider_config::PaymentProviderConfigResponse;
use crate::application::ListPaymentProviderConfigs;
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
pub struct PaginatedConfigsResponse {
    pub items: Vec<PaymentProviderConfigResponse>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

#[utoipa::path(
    get,
    tag = "payment-provider-configs",
    path = "/merchants/{merchant_id}/payment-provider-configs",
    security(("bearer_auth" = [])),
    params(MerchantIdParams, PaginationQuery),
    responses((status = OK, body = PaginatedConfigsResponse))
)]
pub async fn list_payment_provider_configs_handler(
    State(usecase): State<ListPaymentProviderConfigs>,
    Path(params): Path<MerchantIdParams>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedConfigsResponse>, StatusCode> {
    let merchant_id =
        uuid::Uuid::parse_str(&params.merchant_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let result = usecase
        .execute(merchant_id, pagination.page, pagination.page_size)
        .await
        .map_err(|e| {
            eprintln!("Error listing payment provider configs: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let items = result
        .items
        .iter()
        .map(|c| PaymentProviderConfigResponse::from(c.clone()))
        .collect();

    Ok(Json(PaginatedConfigsResponse {
        items,
        total: result.total,
        page: result.page,
        page_size: result.page_size,
    }))
}
