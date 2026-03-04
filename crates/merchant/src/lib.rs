pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod api;

pub use domain::Merchant;
pub use application::{
    CreateMerchant, GetMerchant, UpdateMerchant, DeleteMerchant, ListMerchants,
    MerchantRepository, CreateMerchantRequest, UpdateMerchantRequest, MerchantResponse,
};
pub use infrastructure::MerchantRepositoryPostgres;
pub use api::{router, MerchantState};
