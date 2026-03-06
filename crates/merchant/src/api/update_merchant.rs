use uuid::Uuid;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::application::UpdateMerchant;
use crate::UpdateMerchantRequest;

#[utoipa::path(
    put,
    tag = "merchant",
    security(("bearer_auth" = [])),
    path = "/merchant/{id}",
    request_body = UpdateMerchantRequest,
    responses((status = OK))
)]
pub async fn update_merchant_handler(
    State(usecase): State<UpdateMerchant>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateMerchantRequest>,
) -> Result<Json<&'static str>, StatusCode> {
    // Ensure the ID in the path matches the payload
    if id != payload.id {
        return Err(StatusCode::BAD_REQUEST);
    }

    usecase.execute(payload.id, payload.name, payload.is_active).await
        .map_err(|e| {
            eprintln!("Error updating merchant: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json("Merchant updated successfully"))
}
