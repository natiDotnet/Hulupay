mod extractor;
mod login;
pub mod middleware;
mod register;
mod state;

use std::env;
pub use extractor::AuthUser;
pub use middleware::{authentication, authorization, AuthorizationPolicy};
pub use state::AuthState;

use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::application::{LoginUser, RegisterUser, TokenService};
use crate::infrastructure::JwtTokenService;

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

pub fn router(db: &DatabaseConnection) -> OpenApiRouter {
    // let repo: Arc<dyn UserRepository> = Arc::new(PgUserRepository::new(pool));
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_service: Arc<dyn TokenService> = Arc::new(JwtTokenService::new(jwt_secret));

    let state = AuthState {
        register_use_case: RegisterUser::new(db.clone()),
        login_use_case: LoginUser::new(db.clone(), token_service.clone()),
    };

    OpenApiRouter::new()
        .routes(routes!(register::register_user_handler))
        .routes(routes!(login::login_user_handler))
        .with_state(state)
}

pub fn get_token_service() -> Arc<dyn TokenService> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_service: Arc<dyn TokenService> = Arc::new(JwtTokenService::new(jwt_secret));
    token_service
}
