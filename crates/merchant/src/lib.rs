pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;

pub use api::{MerchantState, router};
pub use application::{
    CreateMerchant, CreateMerchantRequest, DeleteMerchant, GetMerchant, ListMerchants,
    MerchantResponse, UpdateMerchant, UpdateMerchantRequest,
};
pub use domain::Merchant;
// pub use infrastructure::MerchantRepositoryPostgres;
