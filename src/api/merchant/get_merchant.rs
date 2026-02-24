use application::merchant::dto::MerchantResponse;
use application::merchant::get_merchant::GetMerchant;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use crate::api::Error::ApiError;

#[utoipa::path(get, path = "/merchants/{id}", responses((status = OK, body = MerchantResponse)))]
pub async fn get_merchant_handler(State(use_case): State<GetMerchant>,
                             Path(id): Path<Uuid>)
    -> Result<Json<MerchantResponse>, ApiError> {
    let res = use_case.execute(id).await
        .map_err(|_| ApiError::Internal)?
        .map(Json)
        .ok_or(ApiError::NotFound(format!("Merchant with id {} was not found", id)));
    res
}