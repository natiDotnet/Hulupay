mod extractor;
mod login;
mod register;
mod state;
pub mod middleware;

pub use extractor::AuthUser;
pub use state::AuthState;
pub use middleware::{authentication, authorization, AuthorizationPolicy};

use axum::extract::FromRef;
use sqlx::PgPool;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::application::{LoginUser, RegisterUser, TokenService, UserRepository};
use crate::infrastructure::{JwtTokenService, PgUserRepository};

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

pub fn router(pool: PgPool) -> OpenApiRouter {
    let repo: Arc<dyn UserRepository> = Arc::new(PgUserRepository::new(pool));
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_service: Arc<dyn TokenService> = Arc::new(JwtTokenService::new(jwt_secret));
    
    let state = AuthState {
        register_use_case: RegisterUser::new(repo.clone()),
        login_use_case: LoginUser::new(repo.clone(), token_service.clone()),
    };

    OpenApiRouter::new()
        .routes(routes!(register::register_user_handler))
        .routes(routes!(login::login_user_handler))
        .with_state(state)
}
