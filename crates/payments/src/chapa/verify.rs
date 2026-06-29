use crate::PaymentsState;
use crate::chapa::chapa_api_error::ChapaApiErr;
use crate::chapa::checkout_response::{ChapaResponse, ChapaVerifyResponse, Customization};
use crate::domain::{PaymentOrders, payments};
use axum::Json;
use axum::extract::{Path, State};
use axum_macros::debug_handler;
use hulu_core::create_checkout::VerifyPayment;
use hulu_core::gateway_response::VerifyResponse;
use hulu_core::hulu_error::HuluError;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[utoipa::path(
    get,
    tag = "chapa",
    path = "/v1/transaction/verify/{tx_ref}",
    responses((status = OK, body = ChapaResponse<ChapaVerifyResponse>),
        (status = BAD_REQUEST, body = ChapaResponse<serde_json::Value>))
)]
#[debug_handler(state = PaymentsState)]
pub async fn chapa_verify_handler(
    State(verify): State<Arc<dyn VerifyPayment>>,
    State(db): State<DatabaseConnection>,
    Path(tx_ref): Path<String>,
) -> Result<Json<ChapaResponse<ChapaVerifyResponse>>, ChapaApiErr> {
    let provider = verify.execute("master", &tx_ref).await?;

    let response = handle_chapa_verify(provider, &db).await?;
    Ok(Json(response))
}
async fn handle_chapa_verify(
    response: VerifyResponse,
    db: &DatabaseConnection,
) -> Result<ChapaResponse<ChapaVerifyResponse>, HuluError> {
    let (order, customer) = PaymentOrders::find_by_order_ref(&response.reference)
        .find_also_related(payments::payment_customer::Entity)
        .one(db)
        .await
        .map_err(|_| HuluError::ConnectionError)?
        .ok_or_else(|| HuluError::ProviderNotFound)?;
    let customer = customer.ok_or(HuluError::ProviderNotFound)?;

    let verify = ChapaVerifyResponse {
        email: customer.email,
        currency: order.currency,
        amount: response.amount,
        charge: response.charge,
        status: response.status,
        reference: response.id,
        mode: "live".into(),
        first_name: customer.name.clone(),
        last_name: customer.name.clone(),
        method: response.payment_method.into(),
        r#type: "API".into(),
        tx_ref: response.reference,
        created_at: order.created_at,
        updated_at: order.updated_at,
        customization: Customization {
            title: "order".into(),
            description: "description".into(),
            logo: None,
        },
        meta: None,
    };
    Ok(ChapaResponse::<ChapaVerifyResponse> {
        status: "success".into(),
        message: "success".into(),
        data: Some(verify),
    })
}
