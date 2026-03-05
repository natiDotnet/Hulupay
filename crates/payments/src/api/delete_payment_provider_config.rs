use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use utoipa::IntoParams;
use crate::application::DeletePaymentProviderConfig;

#[derive(Deserialize, IntoParams)]
pub struct ConfigIdParams {
    pub id: String,
}

#[utoipa::path(
    delete,
    tag = "payment-provider-configs",
    path = "/payment-provider-configs/{id}",
    security(("bearer_auth" = [])),
    params(ConfigIdParams),
    responses((status = NO_CONTENT))
)]
pub async fn delete_payment_provider_config_handler(
    State(usecase): State<DeletePaymentProviderConfig>,
    Path(params): Path<ConfigIdParams>,
) -> Result<StatusCode, StatusCode> {
    let id = uuid::Uuid::parse_str(&params.id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    usecase.execute(id).await
        .map_err(|e| {
            eprintln!("Error deleting payment provider config: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}
