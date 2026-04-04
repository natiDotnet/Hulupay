use crate::application::CreatePaymentProvider;
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreatePaymentProviderRequest {
    pub code: String,
    pub name: String,
    #[serde(default = "default_is_active")]
    pub is_active: bool,
}

fn default_is_active() -> bool {
    true
}

#[utoipa::path(
    post,
    tag = "payment-providers",
    path = "/payment-providers",
    request_body = CreatePaymentProviderRequest,
    responses((status = CREATED, body = ProviderResponse))
)]
pub async fn create_payment_provider_handler(
    State(usecase): State<CreatePaymentProvider>,
    Json(payload): Json<CreatePaymentProviderRequest>,
) -> Result<Json<ProviderResponse>, StatusCode> {
    let provider = usecase
        .execute(payload.code, payload.name, payload.is_active)
        .await
        .map_err(|e| {
            eprintln!("Error creating payment provider: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ProviderResponse {
        id: provider.id,
        code: provider.code,
        name: provider.name,
        is_active: provider.is_active,
        created_at: provider.created_at,
    }))
}

#[derive(Serialize, ToSchema)]
pub struct ProviderResponse {
    pub id: uuid::Uuid,
    pub code: String,
    pub name: String,
    pub is_active: bool,
    #[serde(with = "time::serde::rfc3339")]
    #[schema(value_type = String, format = DateTime)]
    pub created_at: OffsetDateTime,
}
