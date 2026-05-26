// use crate::application::user_repository::UserRepository;
// use crate::domain::user::User;
// use crate::Role;
// use async_trait::async_trait;
// use sqlx::PgPool;
//
// pub struct PgUserRepository {
//     pool: PgPool,
// }
//
// impl PgUserRepository {
//     pub fn new(pool: PgPool) -> Self {
//         Self { pool }
//     }
// }
//
// #[async_trait]
// impl UserRepository for PgUserRepository {
//     async fn exists_by_email(&self, email: &str) -> anyhow::Result<bool> {
//         let exists: (bool,) =
//             sqlx::query_as("SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)")
//                 .bind(email)
//                 .fetch_one(&self.pool)
//                 .await?;
//
//         Ok(exists.0)
//     }
//
//     async fn insert(&self, user: &User) -> anyhow::Result<()> {
//         sqlx::query(
//             r#"
//             INSERT INTO users (id, email, password_hash, role, merchant_id, created_at, updated_at)
//             VALUES ($1, $2, $3, $4, $5, $6, $7)
//             "#,
//         )
//         .bind(user.id)
//         .bind(&user.email)
//         .bind(&user.password_hash)
//         .bind(user.role.to_string())
//         .bind(user.merchant_id)
//         .bind(user.created_at)
//         .bind(user.updated_at)
//         .execute(&self.pool)
//         .await?;
//
//         Ok(())
//     }
//
//     async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
//         let row = sqlx::query!(
//             r#"select id, email, password_hash, role, merchant_id, created_at, updated_at
//                 from users where email = $1
//                 "#,
//             email
//         )
//         .fetch_optional(&self.pool)
//         .await?;
//
//         Ok(row.map(|r| {
//             let role = Role::from_string(&r.role).unwrap_or(Role::MerchantAdmin);
//
//             User {
//                 id: r.id,
//                 email: r.email,
//                 password_hash: r.password_hash,
//                 role,
//                 merchant_id: r.merchant_id,
//                 is_active: true,
//                 created_at: r.created_at,
//                 updated_at: r.updated_at,
//             }
//         }))
//     }
// }
