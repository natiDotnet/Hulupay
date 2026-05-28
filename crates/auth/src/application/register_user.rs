use crate::application::login_request::{RegisterUserRequest, RegisterUserResponse};
use crate::application::password::hash_password;
use crate::domain::user;
use crate::Role;
use sea_orm::{ActiveModelTrait, DatabaseConnection, SelectExt, Set};

#[derive(Clone)]
pub struct RegisterUser {
    db: DatabaseConnection,
}

impl RegisterUser {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        request: RegisterUserRequest,
    ) -> anyhow::Result<RegisterUserResponse> {
        let exists = user::Entity::find_by_email(&request.email)
            .exists(&self.db)
            .await?;
        if exists {
            return Err(anyhow::anyhow!("User already exists"));
        }

        let password_hash = hash_password(&request.password)?;
        let role =
            Role::from_string(&request.role).ok_or_else(|| anyhow::anyhow!("Invalid role"))?;

        let user = user::ActiveModel {
            email: Set(request.email),
            password_hash: Set(password_hash),
            role: Set(role),
            merchant_id: Set(request.merchant_id),
            ..ActiveModelTrait::default()
        };
        let user = user.insert(&self.db).await?;

        Ok(RegisterUserResponse {
            id: user.id,
            email: user.email,
            role: user.role.to_string(),
        })
    }
}
