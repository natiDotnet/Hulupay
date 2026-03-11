use crate::application::UpdatePaymentProviderConfig;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Deserialize, IntoParams)]
pub struct ConfigIdParams {
    pub id: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdatePaymentProviderConfigRequest {
    pub merchant_id: Uuid,
    pub provider_id: Uuid,
    pub is_test_mode: bool,
    pub config: serde_json::Value,
    pub is_active: bool,
}

#[utoipa::path(
    put,
    tag = "payment-provider-configs",
    path = "/payment-provider-configs/{id}",
    security(("bearer_auth" = [])),
    params(ConfigIdParams),
    request_body = UpdatePaymentProviderConfigRequest,
    responses((status = NO_CONTENT))
)]
pub async fn update_payment_provider_config_handler(
    State(usecase): State<UpdatePaymentProviderConfig>,
    Path(params): Path<ConfigIdParams>,
    Json(payload): Json<UpdatePaymentProviderConfigRequest>,
) -> Result<StatusCode, StatusCode> {
    let id = uuid::Uuid::parse_str(&params.id).map_err(|_| StatusCode::BAD_REQUEST)?;

    usecase
        .execute(
            id,
            payload.merchant_id,
            payload.provider_id,
            payload.is_test_mode,
            payload.config,
            payload.is_active,
        )
        .await
        .map_err(|e| {
            eprintln!("Error updating payment provider config: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}
