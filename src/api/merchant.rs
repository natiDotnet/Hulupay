use utoipa_axum::router::UtoipaMethodRouterExt;
pub mod create_merchant;

use crate::api::merchant::create_merchant::create_merchant_handler;
use crate::merchant::create_merchant::__path_create_merchant_handler;
use application::merchant::create_merchant::CreateMerchant;
use application::merchant::dto::{CreateMerchantRequest, MerchantResponse};
use application::merchant::repository::MerchantRepository;
use axum::handler::Handler;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_axum::router::UtoipaMethodRouter;
use utoipa_axum::routes;

pub fn router(repo: Arc<dyn MerchantRepository + Send + Sync>) -> UtoipaMethodRouter {
    let create_usecase = CreateMerchant::new(repo); // concrete use case
    routes!(create_merchant_handler)
        .with_state(create_usecase)
}