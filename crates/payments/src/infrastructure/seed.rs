use crate::domain;
use auth::Role;
use chrono::Utc;
use domain::payment_provider;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;
use auth::domain::status::AccountStatus;
use hulu_core::create_slug;
use merchant::domain::merchant_status::MerchantStatus;

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

        self.seed_master_merchant(merchant_id).await?;
        self.seed_admin_user(merchant_id).await?;
        self.seed_providers().await?;

        Ok(())
    }
    async fn seed_master_merchant(&self, merchant_id: Uuid) -> anyhow::Result<()> {
        let name = "master";
        let master = merchant::domain::merchant::Entity::find_by_name(name)
            .one(&self.db)
            .await?;
        if master.is_some() {
            return Ok(());
        }
        let _ = merchant::domain::merchant::ActiveModel {
            id: Set(merchant_id),
            slug: Set(create_slug(name)),
            name: Set(name.to_string()),
            email: Set(format!("{}@gmail.com", name)),
            phone: Set("+251994000000".to_string()),
            website: Set("https://www.hulupay.com".to_string()),
            is_active: Set(true),
            status: Set(MerchantStatus::Active),
            created_at: Set(Utc::now()),
            updated_at: Set(Some(Utc::now())),
        }
        .insert(&self.db)
        .await?;

        Ok(())
    }

    async fn seed_admin_user(&self, merchant_id: Uuid) -> anyhow::Result<()> {
        let email = "admin@gmail.com";
        let user = auth::domain::user::Entity::find_by_email(email)
            .one(&self.db)
            .await?;
        if user.is_some() {
            return Ok(());
        }
        let _ = auth::domain::user::ActiveModel {
            id: Set(Uuid::now_v7()),
            name: Set("admin".to_string()),
            email: Set(email.into()),
            password_hash: Set(auth::application::password::hash_password("admin")?),
            merchant_id: Set(merchant_id),
            role: Set(Role::MasterAdmin),
            is_active: Set(true),
            status: Set(AccountStatus::Active),
            created_at: Set(Utc::now()),
            updated_at: Set(Some(Utc::now())),
        }
        .insert(&self.db)
        .await?;

        Ok(())
    }

    async fn seed_providers(&self) -> anyhow::Result<()> {
        let arifpay = payment_provider::ActiveModel {
            name: Set(domain::provider::Provider::ArifPay.to_string()),
            code: Set(domain::provider::Provider::ArifPay.to_string()),
            logo: Set("https://dashboard.arifpay.net/logo.png".to_string()),
            ..Default::default()
        };

        let chapa = payment_provider::ActiveModel {
            name: Set(domain::provider::Provider::Chapa.to_string()),
            code: Set(domain::provider::Provider::Chapa.to_string()),
            logo: Set("https://ethiopianlogos.com/logos/chapa/chapa.png".to_string()),
            ..Default::default()
        };
        
        payment_provider::Entity::insert_many([arifpay, chapa])
            .exec(&self.db)
            .await?;

        Ok(())
    }
}
