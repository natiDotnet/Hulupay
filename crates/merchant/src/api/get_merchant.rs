use crate::application::{ApplicationError, GetMerchant};
use crate::MerchantResponse;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

#[utoipa::path(
    get,
    tag = "merchant",
    security(("bearer_auth" = [])),
    path = "/merchants/{id}",
    responses((status = OK, body = MerchantResponse))
)]
pub async fn get_merchant_handler(
    State(usecase): State<GetMerchant>,
    Path(id): Path<Uuid>,
) -> Result<Json<MerchantResponse>, StatusCode> {
    let res = usecase.execute(id).await.map_err(|e| {
        eprintln!("Error getting merchant: {:?}", e);
        match e {
            ApplicationError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    })?;
    Ok(Json(res))
}
