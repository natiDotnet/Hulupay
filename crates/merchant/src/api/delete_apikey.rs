use crate::application::{ApplicationError, DeleteApiKey};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

#[utoipa::path(
    delete,
    tag = "apikey",
    security(("bearer_auth" = [])),
    path = "/apikeys/{id}",
    responses((status = OK))
)]
pub async fn delete_apikey_handler(
    State(usecase): State<DeleteApiKey>,
    Path(id): Path<Uuid>,
) -> Result<Json<&'static str>, StatusCode> {
    usecase.execute(id).await.map_err(|e| {
        eprintln!("Error deleting api key: {:?}", e);
        match e {
            ApplicationError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    })?;

    Ok(Json("Api key deleted successfully"))
}
