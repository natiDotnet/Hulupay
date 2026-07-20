use crate::application::CreatePaymentProvider;
use crate::application::payment_provider::create_payment_provider::CreatePaymentProviderRequest;
use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
    let provider = usecase.execute(payload).await.map_err(|e| {
        eprintln!("Error creating payment provider: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(ProviderResponse {
        id: provider.id,
        code: provider.code,
        name: provider.name,
        logo: provider.logo,
        is_active: provider.is_active,
        created_at: provider.created_at,
    }))
}

#[derive(Serialize, ToSchema)]
pub struct ProviderResponse {
    pub id: uuid::Uuid,
    pub code: String,
    pub name: String,
    pub logo: String,
    pub is_active: bool,
    // #[serde(with = "time::serde::rfc3339")]
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
}
