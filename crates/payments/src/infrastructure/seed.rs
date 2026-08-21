use crate::domain::payment_provider::PaymentProvider;
use crate::util::now_jiff;
use auth::Role;
use auth::domain::status::AccountStatus;
use hulu_core::create_slug;
use merchant::domain::merchant_status::MerchantStatus;
use toasty::Db;
use uuid::Uuid;

pub struct DataSeeder {
    db: Db,
}

impl DataSeeder {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}
impl DataSeeder {
    pub async fn seed(&self) -> anyhow::Result<()> {
        let merchant_id = Uuid::now_v7();
        let simulator_id = Uuid::now_v7();

        self.seed_master_merchant(merchant_id).await?;
        self.seed_simulator_merchant(simulator_id).await?;

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
        let mut db = self.db.clone();
        if let Some(m) = merchant::domain::merchant::Merchant::filter_by_name("master")
            .first()
            .exec(&mut db)
            .await?
        {
            return Ok(m.id);
        }
        Ok(fallback)
    }

    async fn seed_master_merchant(&self, merchant_id: Uuid) -> anyhow::Result<()> {
        let mut db = self.db.clone();
        let name = "master";

        if merchant::Merchant::filter_by_name(name)
            .first()
            .exec(&mut db)
            .await?
            .is_some()
        {
            return Ok(());
        }

        let _ = toasty::create!(merchant::Merchant {
            id: merchant_id,
            slug: create_slug(name),
            name: name.to_string(),
            email: format!("{name}@gmail.com"),
            phone: "+251994000000".to_string(),
            website: "https://www.hulupay.com".to_string(),
            is_active: true,
            status: MerchantStatus::Active,
            created_at: now_jiff(),
        })
        .exec(&mut db)
        .await?;

        Ok(())
    }

    async fn seed_simulator_merchant(&self, merchant_id: Uuid) -> anyhow::Result<()> {
        let mut db = self.db.clone();
        let name = "simulator";

        if merchant::Merchant::filter_by_name(name)
            .first()
            .exec(&mut db)
            .await?
            .is_some()
        {
            return Ok(());
        }

        let email = format!("{name}@gmail.com");

        let _ = toasty::create!(merchant::Merchant {
            id: merchant_id,
            slug: create_slug(name),
            name: name.to_string(),
            email: &email,
            phone: "+251994000000".to_string(),
            website: "https://www.hulupay.com".to_string(),
            is_active: true,
            status: MerchantStatus::Active,
            created_at: now_jiff(),
        })
        .exec(&mut db)
        .await?;
        let password_hash = auth::application::password::hash_password(name)?;

        let _ = toasty::create!(auth::domain::user::User {
            id: Uuid::now_v7(),
            name: (*name).to_string(),
            email,
            password_hash,
            merchant_id,
            role: Role::Owner,
            is_active: true,
            status: AccountStatus::Active,
            created_at: now_jiff(),
        })
        .exec(&mut db)
        .await?;

        Ok(())
    }

    /// Seed one user per `Role` variant, all linked to the master merchant.
    async fn seed_role_users(&self, merchant_id: Uuid) -> anyhow::Result<()> {
        let seed_users: &[(&str, Role, &str, &str)] = &[
            (
                "Master Admin",
                Role::MasterAdmin,
                "master_admin",
                "master_admin",
            ),
            (
                "Merchant Admin",
                Role::MerchantAdmin,
                "merchant_admin",
                "merchant_admin",
            ),
            ("Owner", Role::Owner, "owner", "owner"),
            ("Admin", Role::Admin, "admin", "admin"),
            ("Developer", Role::Developer, "developer", "developer"),
            ("Finance", Role::Finance, "finance", "finance"),
            ("Viewer", Role::Viewer, "viewer", "viewer"),
        ];

        for (name, role, slug, password) in seed_users {
            let email = format!("{slug}@gmail.com");
            let mut db = self.db.clone();

            if auth::domain::user::User::filter_by_email(&email)
                .first()
                .exec(&mut db)
                .await?
                .is_some()
            {
                continue;
            }

            let password_hash = auth::application::password::hash_password(password)?;

            let _ = toasty::create!(auth::domain::user::User {
                id: Uuid::now_v7(),
                name: (*name).to_string(),
                email,
                password_hash,
                merchant_id,
                role: role.clone(),
                is_active: true,
                status: AccountStatus::Active,
                created_at: now_jiff(),
            })
            .exec(&mut db)
            .await?;
        }

        Ok(())
    }

    async fn seed_providers(&self) -> anyhow::Result<()> {
        let mut db = self.db.clone();
        let now = crate::util::now_jiff();

        let _ = toasty::create!(PaymentProvider {
            name: crate::domain::provider::Provider::Simulator.to_string(),
            code: crate::domain::provider::Provider::Simulator.to_string(),
            logo: "https://ethiopianlogos.com/logos/chapa/chapa.png".to_string(),
            is_active: true,
            created_at: now,
        })
        .exec(&mut db)
        .await?;

        let _ = toasty::create!(PaymentProvider {
            name: crate::domain::provider::Provider::ArifPay.to_string(),
            code: crate::domain::provider::Provider::ArifPay.to_string(),
            logo: "https://dashboard.arifpay.net/logo.png".to_string(),
            is_active: true,
            created_at: now,
        })
        .exec(&mut db)
        .await?;

        let _ = toasty::create!(PaymentProvider {
            name: crate::domain::provider::Provider::Chapa.to_string(),
            code: crate::domain::provider::Provider::Chapa.to_string(),
            logo: "https://ethiopianlogos.com/logos/chapa/chapa.png".to_string(),
            is_active: true,
            created_at: now,
        })
        .exec(&mut db)
        .await?;

        Ok(())
    }
}
