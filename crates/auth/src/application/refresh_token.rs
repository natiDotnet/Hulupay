use crate::application::login_request::{RefreshRequest, RefreshResponse};
use crate::application::permission_service::PermissionService;
use crate::application::token::TokenService;
use crate::domain::refresh_token::RefreshToken;
use crate::util;
use crate::DomainAuthError;
use chrono::Utc;
use std::sync::Arc;

/// Rotation with theft detection:
/// 1. Validate the refresh JWT.
/// 2. Look up the DB row by `jti` (the row id).
/// 3. If the row is already revoked → the family is compromised → revoke all
///    members of the family and return an error (force re-login).
/// 4. Otherwise revoke the current row, mint a new pair, persist the new
///    refresh row (same family_id).
#[derive(Clone)]
pub struct RefreshTokens {
    db: toasty::Db,
    token_service: Arc<dyn TokenService>,
    permission_service: PermissionService,
}

impl RefreshTokens {
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

    pub async fn execute(&self, request: RefreshRequest) -> anyhow::Result<RefreshResponse> {
        let claims = self
            .token_service
            .validate_refresh(&request.refresh_token)?;

        let mut db = self.db.clone();

        let row = RefreshToken::filter_by_id(claims.jti)
            .first()
            .exec(&mut db)
            .await?
            .ok_or(DomainAuthError::InvalidRefreshToken)?;

        // Expired row?
        if row.expires_at < util::now_jiff() {
            return Err(anyhow::anyhow!(DomainAuthError::TokenExpired));
        }

        // Already revoked → token reuse detected. Revoke entire family.
        if row.revoked_at.is_some() {
            self.revoke_family(row.family_id).await;
            return Err(anyhow::anyhow!(DomainAuthError::TokenRevoked));
        }

        // Revoke the current row (rotation).
        let mut current = row.clone();
        toasty::update!(current { revoked_at: util::now_jiff() })
            .exec(&mut db)
            .await?;

        // Mint new tokens.
        let permissions = self
            .permission_service
            .get_permissions_for_role_str(&claims.role)
            .await;

        let access_token = self.token_service.generate_access(
            claims.sub,
            &claims.email,
            &claims.role,
            claims.merchant_id,
            permissions,
        )?;

        let new_id = uuid::Uuid::now_v7();
        let refresh_ttl_secs: i64 = std::env::var("JWT_REFRESH_TTL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(604_800);

        let new_refresh = self.token_service.generate_refresh(
            claims.sub,
            &claims.email,
            &claims.role,
            claims.merchant_id,
            new_id,
        )?;

        let expires_at =
            util::to_jiff(Utc::now() + chrono::Duration::seconds(refresh_ttl_secs));
        toasty::create!(RefreshToken {
            id: new_id,
            user_id: claims.sub,
            token_hash: crate::infrastructure::hash_token(&new_refresh),
            family_id: row.family_id,
            expires_at,
            created_at: util::to_jiff(Utc::now()),
        })
        .exec(&mut db)
        .await?;

        Ok(RefreshResponse {
            access_token,
            refresh_token: new_refresh,
        })
    }

    /// Revoke every refresh token in the given family.
    async fn revoke_family(&self, family_id: uuid::Uuid) {
        let mut db = self.db.clone();

        let rows = RefreshToken::filter(RefreshToken::fields().family_id().eq(family_id))
            .filter(RefreshToken::fields().revoked_at().is_none())
            .exec(&mut db)
            .await;

        if let Ok(rows) = rows {
            let now = util::now_jiff();
            for mut row in rows {
                let _ = toasty::update!(row { revoked_at: now })
                    .exec(&mut db)
                    .await;
            }
        }
    }
}
