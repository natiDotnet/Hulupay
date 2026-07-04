use crate::arifpay::payment_response::{ArifResponse, ArifVerifyResponse};
use crate::chapa::chapa_api_error::ChapaApiErr;
use axum::extract::{Path, State};
use axum::Json;
use hulu_core::create_checkout::VerifyPayment;
use std::sync::Arc;

#[utoipa::path(
    get,
    tag = "arifpay",
    path = "/ms/transaction/status/{session_id}",
    responses((status = OK, body = ArifResponse<ArifVerifyResponse>),
        (status = BAD_REQUEST, body = ArifResponse<serde_json::Value>))
)]
pub async fn arifpay_verify_handler(
    Path(session_id): Path<String>,
    State(verify): State<Arc<dyn VerifyPayment>>,
) -> Result<Json<ArifResponse<ArifVerifyResponse>>, ChapaApiErr> {
    let provider = verify.execute("master", &session_id).await?;

    Ok(Json(provider.into()))
}
