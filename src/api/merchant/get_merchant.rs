use application::merchant::dto::MerchantResponse;
use application::merchant::get_merchant::GetMerchant;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

#[utoipa::path(get, path = "/merchants/{id}", responses((status = OK, body = MerchantResponse)))]
pub async fn get_merchant_handler(State(use_case): State<GetMerchant>,
                             Path(id): Path<Uuid>)
    -> Result<Json<Option<MerchantResponse>>, StatusCode> {
    let res = use_case.execute(id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(res))
}