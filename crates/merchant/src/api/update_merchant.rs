use crate::UpdateMerchantRequest;
use crate::application::{ApplicationError, UpdateMerchant};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

#[utoipa::path(
    put,
    tag = "merchant",
    security(("bearer_auth" = [])),
    path = "/merchants/{id}",
    request_body = UpdateMerchantRequest,
    responses((status = OK))
)]
pub async fn update_merchant_handler(
    State(usecase): State<UpdateMerchant>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateMerchantRequest>,
) -> Result<Json<&'static str>, StatusCode> {
    usecase.execute(id, payload).await.map_err(|e| {
        eprintln!("Error updating merchant: {:?}", e);
        match e {
            ApplicationError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    })?;

    Ok(Json("Merchant updated successfully"))
}
