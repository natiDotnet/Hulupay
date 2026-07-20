use crate::ApiKeyResponse;
use crate::application::ListApiKeys;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

#[utoipa::path(
    get,
    tag = "apikey",
    security(("bearer_auth" = [])),
    path = "/merchants/{id}/apikeys",
    responses((status = OK, body = Vec<ApiKeyResponse>))
)]
pub async fn list_apikeys_handler(
    State(usecase): State<ListApiKeys>,
    Path(merchant_id): Path<Uuid>,
) -> Result<Json<Vec<ApiKeyResponse>>, StatusCode> {
    let res = usecase.execute(merchant_id).await.map_err(|e| {
        eprintln!("Error listing api keys: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(res))
}
