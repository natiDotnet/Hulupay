use auth::Role;
use chrono::Utc;
use merchant::domain;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use uuid::Uuid;

pub struct DataSeeder {
    db: DatabaseConnection,
}

impl DataSeeder {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
impl DataSeeder {
    pub async fn seed(&self) -> anyhow::Result<()> {
        let merchant_id = Uuid::now_v7();

        self.seed_merchant(merchant_id).await?;
        self.seed_user(merchant_id).await?;

        Ok(())
    }
    pub async fn seed_merchant(&self, merchant_id: Uuid) -> anyhow::Result<()> {
        let name = "master";
        let master = domain::merchant::Entity::find_by_name(name)
            .one(&self.db)
            .await?;
        if let Some(_) = master {
            return Ok(());
        }
        let _ = domain::merchant::ActiveModel {
            id: Set(merchant_id),
            name: Set(name.to_string()),
            is_active: Set(true),
            created_at: Set(Utc::now()),
            updated_at: Set(Some(Utc::now())),
        }
        .insert(&self.db)
        .await?;

        Ok(())
    }

    pub async fn seed_user(&self, merchant_id: Uuid) -> anyhow::Result<()> {
        let email = "admin@gmail.com";
        let user = auth::domain::user::Entity::find_by_email(email)
            .one(&self.db)
            .await?;
        if let Some(_) = user {
            return Ok(());
        }
        let _ = auth::domain::user::ActiveModel {
            id: Set(Uuid::now_v7()),
            email: Set(email.into()),
            password_hash: Set(auth::application::password::hash_password("admin")?),
            merchant_id: Set(Some(merchant_id)),
            role: Set(Role::MasterAdmin),
            is_active: Set(true),
            created_at: Set(Utc::now()),
            updated_at: Set(Some(Utc::now())),
        }
        .insert(&self.db)
        .await?;

        Ok(())
    }
}
