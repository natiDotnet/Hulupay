use crate::UpdateApiKeyRequest;
use crate::application::{ApplicationError, UpdateApiKey};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

#[utoipa::path(
    patch,
    tag = "apikey",
    security(("bearer_auth" = [])),
    path = "/apikeys/{id}",
    request_body = UpdateApiKeyRequest,
    responses((status = OK))
)]
pub async fn update_apikey_handler(
    State(usecase): State<UpdateApiKey>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateApiKeyRequest>,
) -> Result<Json<&'static str>, StatusCode> {
    usecase.execute(id, payload).await.map_err(|e| {
        eprintln!("Error updating api key: {:?}", e);
        match e {
            ApplicationError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    })?;

    Ok(Json("Api key updated successfully"))
}
