use crate::application::permission_service::PermissionService;
use crate::application::{
    ChangePassword, ForgotPassword, GetCurrentUser, LoginUser, RefreshTokens, RegisterUser,
    ResetPassword, TokenService, VerifyEmail,
};
use crate::infrastructure::MailService;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    pub db: DatabaseConnection,
    pub token_service: Arc<dyn TokenService>,
    pub register_use_case: RegisterUser,
    pub login_use_case: LoginUser,
    pub permission_service: PermissionService,
    pub verify_email_use_case: VerifyEmail,
    pub refresh_use_case: RefreshTokens,
    pub forgot_password_use_case: ForgotPassword,
    pub reset_password_use_case: ResetPassword,
    pub change_password_use_case: ChangePassword,
    pub get_current_user_use_case: GetCurrentUser,
    pub mail_service: Option<Arc<dyn MailService>>,
}
