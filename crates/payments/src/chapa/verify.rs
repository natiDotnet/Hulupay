use crate::chapa::chapa_api_error::ChapaApiErr;
use crate::chapa::checkout_response::{ChapaResponse, ChapaVerifyResponse};
use axum::extract::{Path, State};
use axum::Json;
use axum_macros::debug_handler;
use hulu_core::create_checkout::VerifyPayment;
use std::sync::Arc;

#[utoipa::path(
    get,
    tag = "chapa",
    path = "/v1/transaction/verify/{tx_ref}",
    responses((status = OK, body = ChapaResponse<ChapaVerifyResponse>),
        (status = BAD_REQUEST, body = ChapaResponse<serde_json::Value>))
)]
#[debug_handler]
pub async fn chapa_verify_handler(
    State(verify): State<Arc<dyn VerifyPayment>>,
    Path(tx_ref): Path<String>,
) -> Result<Json<ChapaResponse<ChapaVerifyResponse>>, ChapaApiErr> {
    let provider = verify.execute("master", &tx_ref).await?;

    Ok(Json(provider.into()))
}
