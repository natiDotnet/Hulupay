use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use auth::api::AuthUser;
use crate::application::GetPaymentProviderConfigByProvider;

#[derive(Deserialize, IntoParams)]
pub struct ProviderNameParams {
    pub provider: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
// #[serde(rename_all = "camelCase")]
pub struct PaymentProviderConfigByProviderResponse {
    pub id: uuid::Uuid,
    pub merchant_id: uuid::Uuid,
    pub provider_id: uuid::Uuid,
    pub is_test_mode: bool,
    pub config: serde_json::Value,
    pub is_active: bool,
    #[serde(with = "time::serde::rfc3339")]
    #[schema(value_type = String, format = DateTime)]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: time::OffsetDateTime,
}

impl From<crate::domain::PaymentProviderConfig> for PaymentProviderConfigByProviderResponse {
    fn from(config: crate::domain::PaymentProviderConfig) -> Self {
        Self {
            id: config.id,
            merchant_id: config.merchant_id,
            provider_id: config.provider_id,
            is_test_mode: config.is_test_mode,
            config: config.config,
            is_active: config.is_active,
            created_at: config.created_at,
            updated_at: config.updated_at,
        }
    }
}

#[utoipa::path(
    get,
    tag = "payment-provider-configs",
    path = "/payment-provider-configs/{provider}/config",
    security(("bearer_auth" = [])),
    params(ProviderNameParams),
    responses((status = OK, body = PaymentProviderConfigByProviderResponse))
)]
pub async fn get_payment_provider_config_by_provider_handler(
    AuthUser(user): AuthUser,
    State(usecase): State<GetPaymentProviderConfigByProvider>,
    Path(params): Path<ProviderNameParams>,
) -> Result<Json<PaymentProviderConfigByProviderResponse>, StatusCode> {
    let config = usecase.execute(user, &params.provider).await
        .map_err(|e| {
            eprintln!("Error getting payment provider config by provider: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(PaymentProviderConfigByProviderResponse::from(config)))
}
