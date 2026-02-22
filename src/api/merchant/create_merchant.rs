use application::merchant::create_merchant::CreateMerchant;
use application::merchant::dto::{CreateMerchantRequest, MerchantResponse};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

#[utoipa::path(post, path = "/merchants", responses((status = OK, body = MerchantResponse)))]
pub async fn create_merchant_handler(State(usecase): State<CreateMerchant>,
                             Json(payload): Json<CreateMerchantRequest>)
    -> Result<Json<MerchantResponse>, StatusCode> {
    let res = usecase.execute(payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(res))
}