use crate::application::UpdatePaymentProvider;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct ProviderIdParams {
    pub id: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdatePaymentProviderRequest {
    pub code: String,
    pub name: String,
    pub is_active: bool,
}

#[utoipa::path(
    put,
    tag = "payment-providers",
    path = "/payment-providers/{id}",
    security(("bearer_auth" = [])),
    params(ProviderIdParams),
    request_body = UpdatePaymentProviderRequest,
    responses((status = NO_CONTENT))
)]
pub async fn update_payment_provider_handler(
    State(usecase): State<UpdatePaymentProvider>,
    Path(params): Path<ProviderIdParams>,
    Json(payload): Json<UpdatePaymentProviderRequest>,
) -> Result<StatusCode, StatusCode> {
    let id = uuid::Uuid::parse_str(&params.id).map_err(|_| StatusCode::BAD_REQUEST)?;

    usecase
        .execute(id, payload.code, payload.name, payload.is_active)
        .await
        .map_err(|e| {
            eprintln!("Error updating payment provider: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}
