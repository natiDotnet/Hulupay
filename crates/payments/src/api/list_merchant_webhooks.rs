use crate::application::ListMerchantWebhooks;
use crate::domain::payments::merchant_webhook::MerchantWebhook;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

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
pub struct MerchantWebhookResponse {
    pub id: Uuid,
    pub payment_order_id: Uuid,
    pub merchant_id: Uuid,
    pub status: String,
    pub provider_reference: String,
    pub payment_method: String,
    pub amount: Decimal,
    pub charge: Decimal,
    pub client_reference: String,
    pub txn_reference: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: jiff::Timestamp,
}

impl From<MerchantWebhook> for MerchantWebhookResponse {
    fn from(webhook: MerchantWebhook) -> Self {
        Self {
            id: webhook.id,
            payment_order_id: webhook.payment_order_id,
            merchant_id: webhook.merchant_id,
            status: webhook.status.to_string(),
            provider_reference: webhook.provider_reference,
            payment_method: webhook.payment_method.to_string(),
            amount: webhook.amount,
            charge: webhook.charge,
            client_reference: webhook.client_reference,
            txn_reference: webhook.txn_reference,
            created_at: webhook.created_at,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedMerchantWebhooksResponse {
    pub items: Vec<MerchantWebhookResponse>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

#[utoipa::path(
    get,
    tag = "merchant-webhooks",
    path = "/merchants/{merchant_id}/webhooks",
    security(("bearer_auth" = [])),
    params(MerchantIdParams, PaginationQuery),
    responses((status = OK, body = PaginatedMerchantWebhooksResponse))
)]
pub async fn list_merchant_webhooks_handler(
    State(usecase): State<ListMerchantWebhooks>,
    Path(params): Path<MerchantIdParams>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedMerchantWebhooksResponse>, StatusCode> {
    let merchant_id =
        uuid::Uuid::parse_str(&params.merchant_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let result = usecase
        .execute(merchant_id, pagination.page, pagination.page_size)
        .await
        .map_err(|e| {
            eprintln!("Error listing merchant webhooks: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let items = result
        .items
        .into_iter()
        .map(MerchantWebhookResponse::from)
        .collect();

    Ok(Json(PaginatedMerchantWebhooksResponse {
        items,
        total: result.total,
        page: result.page,
        page_size: result.page_size,
    }))
}
