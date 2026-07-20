use crate::application::DeletePaymentProvider;
use axum::{
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
    delete,
    tag = "payment-providers",
    path = "/payment-providers/{id}",
    security(("bearer_auth" = [])),
    params(ProviderIdParams),
    responses((status = NO_CONTENT))
)]
pub async fn delete_payment_provider_handler(
    State(usecase): State<DeletePaymentProvider>,
    Path(params): Path<ProviderIdParams>,
) -> Result<StatusCode, StatusCode> {
    let id = uuid::Uuid::parse_str(&params.id).map_err(|_| StatusCode::BAD_REQUEST)?;

    usecase.execute(id).await.map_err(|e| {
        eprintln!("Error deleting payment provider: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::NO_CONTENT)
}
