use crate::application::CreateMerchant;
use crate::{CreateMerchantRequest, MerchantResponse};
use axum::{Json, extract::State, http::StatusCode};

#[utoipa::path(
    post,
    tag = "merchant",
    security(("bearer_auth" = [])),
    path = "/merchants",
    request_body = CreateMerchantRequest,
    responses((status = CREATED, body = MerchantResponse))
)]
pub async fn create_merchant_handler(
    State(usecase): State<CreateMerchant>,
    Json(payload): Json<CreateMerchantRequest>,
) -> Result<Json<MerchantResponse>, StatusCode> {
    let res = usecase.execute(payload).await.map_err(|e| {
        eprintln!("Error creating merchant: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(res))
}
