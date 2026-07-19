mod change_password;
mod extractor;
mod forgot_password;
mod get_current_user;
mod login;
pub mod middleware;
mod logout;
mod refresh;
mod register;
mod reset_password;
mod state;
mod verify_email;

use crate::application::permission_service::PermissionService;
use crate::infrastructure::SmtpMailService;
pub use extractor::AuthUser;
pub use middleware::{authentication, authorization, AuthorizationPolicy};
pub use state::AuthState;

use sea_orm::DatabaseConnection;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::application::TokenService;
use crate::infrastructure::JwtTokenService;

pub fn router(db: &DatabaseConnection) -> OpenApiRouter {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_service: Arc<dyn TokenService> = Arc::new(JwtTokenService::new(jwt_secret));

    let permission_service = PermissionService::new(db.clone());

    // Seed built-in role→permission mappings (idempotent)
    tokio::spawn({
        let svc = permission_service.clone();
        async move {
            svc.seed_builtin_roles().await;
        }
    });

    // Attempt to create SMTP mail service; if env vars are missing, email
    // features are silently disabled.
    let mail_service: Option<Arc<dyn crate::infrastructure::MailService>> =
        match SmtpMailService::from_env() {
            Ok(svc) => Some(Arc::new(svc)),
            Err(_) => {
                tracing::warn!("SMTP not configured — email features disabled");
                None
            }
        };

    let state = AuthState {
        db: db.clone(),
        token_service: token_service.clone(),
        register_use_case: crate::application::RegisterUser::new(db.clone(), mail_service.clone()),
        login_use_case: crate::application::LoginUser::new(
            db.clone(),
            token_service.clone(),
            permission_service.clone(),
        ),
        permission_service: permission_service.clone(),
        verify_email_use_case: crate::application::VerifyEmail::new(db.clone()),
        refresh_use_case: crate::application::RefreshTokens::new(
            db.clone(),
            token_service.clone(),
            permission_service,
        ),
        forgot_password_use_case: crate::application::ForgotPassword::new(db.clone(), mail_service.clone()),
        reset_password_use_case: crate::application::ResetPassword::new(db.clone()),
        change_password_use_case: crate::application::ChangePassword::new(db.clone()),
        get_current_user_use_case: crate::application::GetCurrentUser::new(db.clone()),
        mail_service,
    };

    OpenApiRouter::new()
        // Public routes (no auth required)
        .routes(routes!(register::register_user_handler))
        .routes(routes!(login::login_user_handler))
        .routes(routes!(refresh::refresh_handler))
        .routes(routes!(verify_email::verify_email_handler))
        .routes(routes!(forgot_password::forgot_password_handler))
        .routes(routes!(reset_password::reset_password_handler))
        // Protected routes (require authenticated user)
        .routes(routes!(logout::logout_handler))
        .routes(routes!(change_password::change_password_handler))
        .routes(routes!(get_current_user::get_current_user_handler))
        .with_state(state)
}

pub fn get_token_service() -> Arc<dyn TokenService> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Arc::new(JwtTokenService::new(jwt_secret))
}
