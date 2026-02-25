use application::merchant::dto::{MerchantResponse, UpdateMerchantRequest};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use application::merchant::update_merchant::UpdateMerchant;

#[utoipa::path(put, path = "/merchants/{id}", responses((status = OK, body = MerchantResponse)))]
pub async fn update_merchant_handler(State(use_case): State<UpdateMerchant>,
                                     Path(id): Path<Uuid>,
                                     Json(payload): Json<UpdateMerchantRequest>)
                                     -> Result<(), StatusCode> {
    use_case.execute(id, payload.name, payload.is_active).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}