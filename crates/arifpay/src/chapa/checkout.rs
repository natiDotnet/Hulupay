use crate::arifpay::payment_request::ArifpayPaymentRequest;
use crate::chapa::checkout_request::ChapaInitializeRequest;
use crate::chapa::ChapaState;
use axum::extract::State;
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderMap;
use axum::{http::StatusCode, Json};
use axum_macros::debug_handler;
use core::payment_request::PaymentRequest;
use payments::domain::payment_order::Model;
use payments::domain::payment_status::{PaymentStatus, TxDirection, TxStatus};
use payments::domain::provider::Provider;
use sea_orm::{ActiveModelTrait, Set};
use serde_json::{json, Value};
use uuid::Uuid;

#[utoipa::path(
    post,
    tag = "chapa",
    path = "/v1/transaction/initialize",
    responses((status = OK, body = ChapaInitializeRequest))
)]
#[debug_handler]
pub async fn checkout_handler(
    State(chapa_state): State<ChapaState>,
    headers: HeaderMap,
    Json(payload): Json<ChapaInitializeRequest>,
) -> (StatusCode, Json<Value>) {
    let token = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim);
    let request: PaymentRequest = payload.into();
    execute(chapa_state, request, token).await
}

const ARIFPAY_GATEWAY_URL: &str = "https://gateway.arifpay.org";
pub async fn execute(
    chapa_state: ChapaState,
    request: PaymentRequest,
    token: Option<&str>,
) -> (StatusCode, Json<Value>) {
    let Some(token) = token else {
        return failed(StatusCode::UNAUTHORIZED, "Authorization required");
    };
    let reference = request.payment.reference.clone();
    let req = request.clone();
    let payload: ArifpayPaymentRequest = request.into();

    let response = chapa_state
        .arifpay_service
        .create_session(ARIFPAY_GATEWAY_URL.to_string(), token.to_string(), &payload)
        .await;

    match response {
        Ok(arifpay) if arifpay.success => {
            let order: Option<Model> = payments::domain::payment_order::ActiveModel {
                order_ref: Set(reference),
                status: Set(PaymentStatus::Pending),
                amount: Set(req.payment.amount),
                currency: Set(req.payment.currency.clone()),
                provider: Set(Provider::ArifPay),
                idempotency_key: Set(req.payment.reference.clone()),
                retry_count: Set(0),
                merchant_id: Set(Uuid::now_v7()),
                ..Default::default()
            }
            .insert(&chapa_state.db)
            .await
            .ok();

            let order = match order {
                None => {
                    return failed(StatusCode::BAD_REQUEST, "failed to persist the state");
                }
                Some(or) => or,
            };
            let transaction = payments::domain::payment_transaction::ActiveModel {
                provider: Set(Provider::ArifPay),
                currency: Set(req.payment.currency.clone()),
                status: Set(TxStatus::Pending),
                amount: Set(req.payment.amount),
                direction: Set(TxDirection::Charge),
                provider_tx_id: Set(Some(req.payment.reference.clone())),
                provider_response: Set(arifpay.row_response),
                payment_order_id: Set(order.id),
                ..Default::default()
            }
            .insert(&chapa_state.db)
            .await;

            match arifpay.data.map(|d| d.payment_url) {
                Some(checkout_url) => (
                    StatusCode::OK,
                    Json(json!({
                        "message": "Hosted Link",
                        "status": "success",
                        "data": { "checkout_url": checkout_url }
                    })),
                ),
                None => failed(
                    StatusCode::BAD_REQUEST,
                    "Arifpay returned success but no payment URL",
                ),
            }
        }
        Ok(arifpay) => failed(StatusCode::BAD_REQUEST, &arifpay.message),
        Err(err) => failed(StatusCode::BAD_REQUEST, &err.to_string()),
    }
}

fn failed(status: StatusCode, message: &str) -> (StatusCode, Json<Value>) {
    (
        status,
        Json(json!({
            "message": message,
            "status": "failed",
            "data": Value::Null,
        })),
    )
}
