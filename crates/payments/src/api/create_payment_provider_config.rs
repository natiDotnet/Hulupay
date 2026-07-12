use crate::application::payment_provider_config::create_payment_provider_config::CreatePaymentProviderConfigRequest;
use crate::application::CreatePaymentProviderConfig;
use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::domain::environment::Environment;

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
    let config = usecase.execute(payload).await.map_err(|e| {
        eprintln!("Error creating payment provider config: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(PaymentProviderConfigResponse::from(config)))
}

#[derive(Serialize, ToSchema)]
pub struct PaymentProviderConfigResponse {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub provider_id: Uuid,
    pub environment: Environment,
    pub priority: i32,
    // pub is_test_mode: bool,
    pub config: serde_json::Value,
    pub is_active: bool,
    pub is_default: bool,
    // #[serde(with = "time::serde::rfc3339")]
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    // #[serde(with = "time::serde::rfc3339")]
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
}

impl From<crate::domain::PaymentProviderConfig> for PaymentProviderConfigResponse {
    fn from(config: crate::domain::PaymentProviderConfig) -> Self {
        Self {
            id: config.id,
            merchant_id: config.merchant_id,
            provider_id: config.provider_id,
            priority: config.priority,
            environment: config.environment,
            // is_test_mode: config,
            config: config.config,
            is_active: config.is_active,
            is_default: config.is_default,
            created_at: config.created_at,
            updated_at: config.updated_at,
        }
    }
}
