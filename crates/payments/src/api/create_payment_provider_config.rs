use crate::application::CreatePaymentProviderConfig;
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct CreatePaymentProviderConfigRequest {
    pub merchant_id: Uuid,
    pub provider_id: Uuid,
    #[serde(default = "default_is_test_mode")]
    pub is_test_mode: bool,
    pub config: serde_json::Value,
    #[serde(default = "default_is_active")]
    pub is_active: bool,
}

fn default_is_test_mode() -> bool {
    false
}

fn default_is_active() -> bool {
    true
}

#[derive(Serialize, ToSchema)]
pub struct PaymentProviderConfigResponse {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub provider_id: Uuid,
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

impl From<crate::domain::PaymentProviderConfig> for PaymentProviderConfigResponse {
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
    post,
    tag = "payment-provider-configs",
    path = "/payment-provider-configs",
    security(("bearer_auth" = [])),
    request_body = CreatePaymentProviderConfigRequest,
    responses((status = CREATED, body = PaymentProviderConfigResponse))
)]
pub async fn create_payment_provider_config_handler(
    State(usecase): State<CreatePaymentProviderConfig>,
    Json(payload): Json<CreatePaymentProviderConfigRequest>,
) -> Result<Json<PaymentProviderConfigResponse>, StatusCode> {
    let config = usecase.execute(
        payload.merchant_id,
        payload.provider_id,
        payload.is_test_mode,
        payload.config,
        payload.is_active,
    ).await
        .map_err(|e| {
            eprintln!("Error creating payment provider config: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(PaymentProviderConfigResponse::from(config)))
}
