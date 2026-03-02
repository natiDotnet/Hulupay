use std::env;
use crate::api::auth::register::__path_register_user_handler;
use crate::api::auth::login::__path_login_user_handler;
use std::sync::Arc;
use axum::extract::{FromRef, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use application::encryption::EncryptionService;
use application::payments::arifpay::config_request_dto::ArifPayConfigRequest;
use domain::user::Role;
use infrastructure::payments::config_service::ProviderConfigService;

pub mod auth_user;
mod login;
mod register;
pub mod extractor;

#[derive(Clone)]
pub struct AuthState {
    register_use_case: RegisterUser,
    login_use_case: LoginUser,
}

impl FromRef<AuthState> for RegisterUser {
    fn from_ref(state: &AuthState) -> Self {
        state.register_use_case.clone()
    }
}
impl FromRef<AuthState> for LoginUser {
    fn from_ref(state: &AuthState) -> Self {
        state.login_use_case.clone()
    }
}
pub fn router(pool: Pool<Postgres>) -> OpenApiRouter {
    let repo: Arc<dyn UserRepository> = Arc::new(PgUserRepository::new(pool));
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_service: Arc<dyn TokenService> = Arc::new(JwtTokenService::new(jwt_secret));
    let state = AuthState {
        register_use_case: RegisterUser::new(repo.clone()),
        login_use_case: LoginUser::new(repo.clone(), token_service.clone()),
    };

    OpenApiRouter::new()
        .routes(routes!(register_user_handler))
        .routes(routes!(login_user_handler))
        .with_state(state)
}

// async fn set_arifpay_config(
//     auth: AuthUser,
//     State(service): State<Arc<ProviderConfigService<impl EncryptionService>>>,
//     Json(request): Json<ArifPayConfigRequest>,
// ) -> Result<Json<&'static str>, StatusCode> {
//
//     let merchant_id = match auth.role {
//         Role::MasterAdmin => {
//             return Err(StatusCode::BAD_REQUEST);
//         }
//         Role::MerchantAdmin => auth.merchant_id.unwrap(),
//     };
//
//     service
//         .set_arifpay_config(request)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//
//     Ok(Json("Config saved"))
// }

use sqlx::{PgPool, Pool, Postgres};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use application::auth::login::LoginUser;
use application::auth::register_user::RegisterUser;
use application::auth::token::TokenService;
use application::auth::user_repository::UserRepository;
use application::merchant::create_merchant::CreateMerchant;
use application::merchant::delete_merchant::DeleteMerchant;
use application::merchant::get_merchant::GetMerchant;
use application::merchant::list_merchants::ListMerchants;
use application::merchant::repository::MerchantRepository;
use application::merchant::update_merchant::UpdateMerchant;
use infrastructure::auth::jwt_token_service::JwtTokenService;
use infrastructure::auth::pg_user_repository::PgUserRepository;
use infrastructure::persistence::merchant_repository_impl::MerchantRepositoryPostgres;
use crate::api::auth::extractor::AuthUser;
use crate::api::auth::login::login_user_handler;
use crate::api::auth::register::register_user_handler;

pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
}