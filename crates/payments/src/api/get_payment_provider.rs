use super::create_payment_provider::ProviderResponse;
use crate::application::GetPaymentProvider;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct ProviderIdParams {
    pub id: String,
}

#[utoipa::path(
    get,
    tag = "payment-providers",
    path = "/payment-providers/{id}",
    security(("bearer_auth" = [])),
    params(ProviderIdParams),
    responses((status = OK, body = ProviderResponse))
)]
pub async fn get_payment_provider_handler(
    State(usecase): State<GetPaymentProvider>,
    Path(params): Path<ProviderIdParams>,
) -> Result<Json<ProviderResponse>, StatusCode> {
    let id = uuid::Uuid::parse_str(&params.id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let provider = usecase
        .execute(id)
        .await
        .map_err(|e| {
            eprintln!("Error getting payment provider: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ProviderResponse {
        id: provider.id,
        code: provider.code,
        name: provider.name,
        is_active: provider.is_active,
        created_at: provider.created_at,
    }))
}
