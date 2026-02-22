use application::merchant::delete_merchant::DeleteMerchant;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use uuid::Uuid;

#[utoipa::path(delete, path = "/merchants/{id}", responses((status = OK, body = ())))]
pub async fn delete_merchant_handler(State(use_case): State<DeleteMerchant>,
                                     Path(id): Path<Uuid>)
                                     -> Result<(), StatusCode> {
    let res = use_case.execute(id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}