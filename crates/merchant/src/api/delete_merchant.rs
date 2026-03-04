use uuid::Uuid;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::application::DeleteMerchant;

#[utoipa::path(
    delete,
    tag = "merchant",
    path = "/merchant/{id}",
    responses((status = OK))
)]
pub async fn delete_merchant_handler(
    State(usecase): State<DeleteMerchant>,
    Path(id): Path<Uuid>,
) -> Result<Json<&'static str>, StatusCode> {
    usecase.execute(id).await
        .map_err(|e| {
            eprintln!("Error deleting merchant: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json("Merchant deleted successfully"))
}
