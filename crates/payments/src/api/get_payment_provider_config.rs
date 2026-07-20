use super::create_payment_provider_config::PaymentProviderConfigResponse;
use crate::application::GetPaymentProviderConfig;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct ConfigIdParams {
    pub id: String,
}

#[utoipa::path(
    get,
    tag = "payment-provider-configs",
    path = "/payment-provider-configs/{id}",
    security(("bearer_auth" = [])),
    params(ConfigIdParams),
    responses((status = OK, body = PaymentProviderConfigResponse))
)]
pub async fn get_payment_provider_config_handler(
    State(usecase): State<GetPaymentProviderConfig>,
    Path(params): Path<ConfigIdParams>,
) -> Result<Json<PaymentProviderConfigResponse>, StatusCode> {
    let id = uuid::Uuid::parse_str(&params.id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let config = usecase
        .execute(id)
        .await
        .map_err(|e| {
            eprintln!("Error getting payment provider config: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(PaymentProviderConfigResponse::from(config)))
}
