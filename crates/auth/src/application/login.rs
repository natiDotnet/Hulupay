use crate::application::login_request::{LoginRequest, LoginResponse};
use crate::application::password::verify_password;
use crate::application::permission_service::PermissionService;
use crate::application::token::TokenService;
use crate::domain::refresh_token::RefreshToken;
use crate::domain::user::User;
use crate::DomainAuthError;
use crate::util;
use chrono::Utc;
use std::sync::Arc;

#[derive(Clone)]
pub struct LoginUser {
    db: toasty::Db,
    token_service: Arc<dyn TokenService>,
    permission_service: PermissionService,
}

impl LoginUser {
    pub fn new(
        db: toasty::Db,
        token_service: Arc<dyn TokenService>,
        permission_service: PermissionService,
    ) -> Self {
        Self {
            db,
            token_service,
            permission_service,
        }
    }

    pub async fn execute(&self, request: LoginRequest) -> anyhow::Result<LoginResponse> {
        let mut db = self.db.clone();

        let user = User::filter_by_email(&request.email)
            .first()
            .exec(&mut db)
            .await?
            .ok_or(DomainAuthError::InvalidCredentials)?;

        let is_password_valid = verify_password(&user.password_hash, &request.password);
        if !is_password_valid {
            return Err(anyhow::anyhow!(DomainAuthError::InvalidCredentials));
        }

        let permissions = self
            .permission_service
            .get_permissions_for_role(&user.role)
            .await;

        let access_token = self.token_service.generate_access(
            user.id,
            &user.email,
            &user.role.to_string(),
            user.merchant_id,
            permissions,
        )?;

        // Mint refresh token and persist it in the DB.
        let refresh_id = uuid::Uuid::now_v7();
        let family_id = uuid::Uuid::now_v7();
        let refresh_ttl_secs: i64 = std::env::var("JWT_REFRESH_TTL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(604_800);

        let refresh_token_jwt = self.token_service.generate_refresh(
            user.id,
            &user.email,
            &user.role.to_string(),
            user.merchant_id,
            refresh_id,
        )?;

        let expires_at =
            util::to_jiff(Utc::now() + chrono::Duration::seconds(refresh_ttl_secs));
        toasty::create!(RefreshToken {
            id: refresh_id,
            user_id: user.id,
            token_hash: crate::infrastructure::hash_token(&refresh_token_jwt),
            family_id,
            expires_at,
            created_at: util::to_jiff(Utc::now()),
        })
        .exec(&mut db)
        .await?;

        Ok(LoginResponse {
            access_token,
            refresh_token: refresh_token_jwt,
        })
    }
}
