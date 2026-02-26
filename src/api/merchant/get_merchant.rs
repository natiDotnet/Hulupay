use crate::api::error::ApiError;
use application::merchant::dto::MerchantResponse;
use application::merchant::get_merchant::GetMerchant;
use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

#[utoipa::path(get, path = "/merchants/{id}", params(
        ("id" = Uuid, Path, description = "Merchant id")
), responses((status = OK, body = MerchantResponse)))]
pub async fn get_merchant_handler(Path(id): Path<Uuid>,
                                  State(use_case): State<GetMerchant>
                             )
    -> Result<Json<MerchantResponse>, ApiError> {
    let res = use_case.execute(id).await
        .map(Json)
        .map_err(|err| err.into());
    res
}