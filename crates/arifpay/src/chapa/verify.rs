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

    // let provider = ArifPayInitializeResponse {
    //     error: false,
    //     msg: "".to_string(),
    //     data: Some(ArifPayInitializeData {
    //         session_id: "qwertyuiop".to_string(),
    //         payment_url: "qwertyuiop".to_string(),
    //         cancel_url: "QWERTYUIOP".to_string(),
    //         total_amount: Decimal::from_f64_retain(11.1).unwrap(),
    //     }),
    // };

    Ok(Json(provider.into()))
}
