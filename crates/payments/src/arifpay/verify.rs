use crate::arifpay::payment_response::{ArifResponse, ArifVerifyResponse};
use crate::chapa::chapa_api_error::ChapaApiErr;
use axum::Json;
use axum::extract::{Path, State};
use hulu_core::create_checkout::VerifyPayment;
use std::sync::Arc;
use auth::api::AuthUser;

#[utoipa::path(
    get,
    tag = "arifpay",
    path = "/ms/transaction/status/{session_id}",
    responses((status = OK, body = ArifResponse<ArifVerifyResponse>),
        (status = BAD_REQUEST, body = ArifResponse<serde_json::Value>))
)]
pub async fn arifpay_verify_handler(
    AuthUser(ctx): AuthUser,
    Path(session_id): Path<String>,
    State(verify): State<Arc<dyn VerifyPayment>>,
) -> Result<Json<ArifResponse<ArifVerifyResponse>>, ChapaApiErr> {
    let provider = verify.execute(ctx.merchant_id, &session_id).await?;

    Ok(Json(provider.into()))
}
