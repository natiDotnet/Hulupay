use crate::domain;
use auth::Role;
use auth::domain::status::AccountStatus;
use chrono::Utc;
use domain::payment_provider;
use hulu_core::create_slug;
use merchant::domain::merchant_status::MerchantStatus;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
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

        self.seed_master_merchant(merchant_id).await?;

        // Resolve the real master merchant id so re-runs link users to the
        // existing merchant rather than a freshly-generated UUID.
        let master_merchant_id = self.get_master_merchant_id(merchant_id).await?;

        // Seed one user per role for development/testing.
        self.seed_role_users(master_merchant_id).await?;
        self.seed_providers().await?;

        Ok(())
    }

    /// Look up the master merchant by name, falling back to the provided id.
    async fn get_master_merchant_id(&self, fallback: Uuid) -> anyhow::Result<Uuid> {
        if let Some(m) = merchant::domain::merchant::Entity::find_by_name("master")
            .one(&self.db)
            .await?
        {
            return Ok(m.id);
        }
        Ok(fallback)
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

    /// Seed one user per `Role` variant, all linked to the master merchant.
    ///
    /// Credentials follow the pattern:
    ///   - email:    `{role_slug}@gmail.com` (e.g. `developer@gmail.com`)
    ///   - password: `{role_slug}`            (e.g. `developer`)
    ///
    /// Each user is created only if no user with that email exists yet.
    async fn seed_role_users(&self, merchant_id: Uuid) -> anyhow::Result<()> {
        // (display name, role, email slug, password)
        // MasterAdmin is seeded here too so everything lives in one loop.
        let seed_users: &[(&str, Role, &str, &str)] = &[
            ("Master Admin",   Role::MasterAdmin,   "master_admin",   "master_admin"),
            ("Merchant Admin", Role::MerchantAdmin, "merchant_admin", "merchant_admin"),
            ("Owner",          Role::Owner,         "owner",          "owner"),
            ("Admin",          Role::Admin,         "admin",          "admin"),
            ("Developer",      Role::Developer,     "developer",      "developer"),
            ("Finance",        Role::Finance,       "finance",        "finance"),
            ("Viewer",         Role::Viewer,        "viewer",         "viewer"),
        ];

        for (name, role, slug, password) in seed_users {
            let email = format!("{slug}@gmail.com");

            // Skip if the user already exists (idempotent re-run).
            if auth::domain::user::Entity::find()
                .filter(auth::domain::user::Column::Email.eq(&email))
                .one(&self.db)
                .await?
                .is_some()
            {
                continue;
            }

            let password_hash = auth::application::password::hash_password(password)?;

            let _ = auth::domain::user::ActiveModel {
                id: Set(Uuid::now_v7()),
                name: Set((*name).to_string()),
                email: Set(email),
                password_hash: Set(password_hash),
                merchant_id: Set(merchant_id),
                role: Set(role.clone()),
                is_active: Set(true),
                status: Set(AccountStatus::Active),
                email_verified_at: Set(None),
                password_changed_at: Set(None),
                created_at: Set(Utc::now()),
                updated_at: Set(Some(Utc::now())),
            }
            .insert(&self.db)
            .await?;
        }

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
