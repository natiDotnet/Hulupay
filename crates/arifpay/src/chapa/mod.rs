use crate::arifpay::arifpay_service::ArifpayService;
use crate::chapa::checkout::__path_checkout_handler;
use crate::chapa::checkout::checkout_handler;
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub mod checkout;
pub mod checkout_request;
#[derive(Clone)]
pub struct ChapaState {
    arifpay_service: ArifpayService,
    db: DatabaseConnection,
}

impl FromRef<ChapaState> for ArifpayService {
    fn from_ref(state: &ChapaState) -> Self {
        state.arifpay_service.clone()
    }
}
pub fn router(db: &DatabaseConnection) -> OpenApiRouter {
    let client = reqwest::Client::builder()
        // .default_headers(headers)
        .build()
        .unwrap();
    let state = ChapaState {
        arifpay_service: ArifpayService::new(client),
        db: db.clone(),
    };

    OpenApiRouter::new()
        .routes(routes!(checkout_handler))
        // .routes(routes!(login::login_user_handler))
        .with_state(state)
}
