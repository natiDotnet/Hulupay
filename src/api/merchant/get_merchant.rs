use crate::api::error::ApiError;
use application::merchant::dto::MerchantResponse;
use application::merchant::get_merchant::GetMerchant;
use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

#[utoipa::path(get, path = "/merchants/{id}", responses((status = OK, body = MerchantResponse)))]
pub async fn get_merchant_handler(State(use_case): State<GetMerchant>,
                             Path(id): Path<Uuid>)
    -> Result<Json<MerchantResponse>, ApiError> {
    let res = use_case.execute(id).await
        .map(Json)
        .map_err(|err| err.into());
    res
}