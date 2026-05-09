use crate::application::DeleteMerchant;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

#[utoipa::path(
    delete,
    tag = "merchant",
    security(("bearer_auth" = [])),
    path = "/merchants/{id}",
    responses((status = OK))
)]
pub async fn delete_merchant_handler(
    State(usecase): State<DeleteMerchant>,
    Path(id): Path<Uuid>,
) -> Result<Json<&'static str>, StatusCode> {
    usecase.execute(id).await.map_err(|e| {
        eprintln!("Error deleting merchant: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json("Merchant deleted successfully"))
}
