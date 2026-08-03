use crate::application::login_request::{RefreshRequest, RefreshResponse};
use crate::application::permission_service::PermissionService;
use crate::application::token::TokenService;
use crate::domain::refresh_token;
use crate::DomainAuthError;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
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
    db: DatabaseConnection,
    token_service: Arc<dyn TokenService>,
    permission_service: PermissionService,
}

impl RefreshTokens {
    pub fn new(
        db: DatabaseConnection,
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

        let row = refresh_token::Entity::find_by_id(claims.jti)
            .one(&self.db)
            .await?
            .ok_or(DomainAuthError::InvalidRefreshToken)?;

        // Expired row?
        if row.expires_at < Utc::now() {
            return Err(anyhow::anyhow!(DomainAuthError::TokenExpired));
        }

        // Already revoked → token reuse detected. Revoke entire family.
        if row.revoked_at.is_some() {
            self.revoke_family(row.family_id).await;
            return Err(anyhow::anyhow!(DomainAuthError::TokenRevoked));
        }

        // Revoke the current row (rotation).
        let mut current: refresh_token::ActiveModel = row.clone().into();
        current.revoked_at = Set(Some(Utc::now()));
        current.update(&self.db).await?;

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
        let refresh_ttl_secs: u64 = std::env::var("JWT_REFRESH_TTL_SECS")
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

        let refresh_row = refresh_token::ActiveModel {
            id: Set(new_id),
            user_id: Set(claims.sub),
            token_hash: Set(crate::infrastructure::hash_token(&new_refresh)),
            family_id: Set(row.family_id),
            expires_at: Set(Utc::now() + chrono::Duration::seconds(refresh_ttl_secs as i64)),
            revoked_at: Set(None),
            created_at: Set(Utc::now()),
        };
        refresh_row.insert(&self.db).await?;

        Ok(RefreshResponse {
            access_token,
            refresh_token: new_refresh,
        })
    }

    /// Revoke every refresh token in the given family.
    async fn revoke_family(&self, family_id: uuid::Uuid) {
        let rows = refresh_token::Entity::find()
            .filter(refresh_token::Column::FamilyId.eq(family_id))
            .filter(refresh_token::Column::RevokedAt.is_null())
            .all(&self.db)
            .await;

        if let Ok(rows) = rows {
            for row in rows {
                let mut am: refresh_token::ActiveModel = row.into();
                am.revoked_at = Set(Some(Utc::now()));
                let _ = am.update(&self.db).await;
            }
        }
    }
}
