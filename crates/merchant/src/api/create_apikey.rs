use crate::application::{ApplicationError, CreateApiKey};
use crate::{CreateApiKeyRequest, CreateApiKeyResponse};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

#[utoipa::path(
    post,
    tag = "apikey",
    security(("bearer_auth" = [])),
    path = "/merchants/{id}/apikeys",
    request_body = CreateApiKeyRequest,
    responses((status = CREATED, body = CreateApiKeyResponse))
)]
pub async fn create_apikey_handler(
    State(usecase): State<CreateApiKey>,
    Path(merchant_id): Path<Uuid>,
    Json(payload): Json<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<CreateApiKeyResponse>), StatusCode> {
    let res = usecase.execute(merchant_id, payload).await.map_err(|e| {
        eprintln!("Error creating api key: {:?}", e);
        match e {
            ApplicationError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    })?;

    Ok((StatusCode::CREATED, Json(res)))
}
