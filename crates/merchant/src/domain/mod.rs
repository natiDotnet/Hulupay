mod error;
pub mod merchant;
pub mod merchant_status;

// API key entity now lives in the `auth` crate to avoid a duplicate
// definition of the `apikeys` table. Re-export it so existing
// references (`crate::domain::ApiKey`) keep working.
pub use auth::domain::apikey::Model as ApiKey;
pub use auth::domain::apikey;

pub use error::DomainError;
pub use merchant::Merchant;
