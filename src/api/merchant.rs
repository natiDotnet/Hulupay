use utoipa_axum::router::{OpenApiRouter, UtoipaMethodRouterExt};
mod create_merchant;
mod get_merchant;
mod update_merchant;
mod delete_merchant;
mod list_merchants;

use application::merchant::create_merchant::CreateMerchant;
use application::merchant::delete_merchant::DeleteMerchant;
use application::merchant::get_merchant::GetMerchant;
use application::merchant::list_merchants::ListMerchants;
use application::merchant::repository::MerchantRepository;
use application::merchant::update_merchant::UpdateMerchant;
use axum::extract::FromRef;
use std::sync::Arc;
use utoipa_axum::router::UtoipaMethodRouter;
use utoipa_axum::routes;

#[derive(Clone)]
struct MerchantState {
    create_use_case: CreateMerchant,
    get_use_case: GetMerchant,
    update_use_case: UpdateMerchant,
    delete_use_case: DeleteMerchant,
}

impl FromRef<MerchantState> for CreateMerchant {
    fn from_ref(state: &MerchantState) -> Self {
        state.create_use_case.clone()
    }
}

impl FromRef<MerchantState> for GetMerchant {
    fn from_ref(state: &MerchantState) -> Self {
        state.get_use_case.clone()
    }
}

impl FromRef<MerchantState> for UpdateMerchant {
    fn from_ref(state: &MerchantState) -> Self {
        state.update_use_case.clone()
    }
}

impl FromRef<MerchantState> for DeleteMerchant {
    fn from_ref(state: &MerchantState) -> Self {
        state.delete_use_case.clone()
    }
}

pub fn merchant_router(repo: Arc<dyn MerchantRepository>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(router(repo.clone()))
        .routes(merchant_list(repo.clone()))
}
fn router(repo: Arc<dyn MerchantRepository>) -> UtoipaMethodRouter {

    let state = MerchantState {
        create_use_case: CreateMerchant::new(repo.clone()),
        get_use_case: GetMerchant::new(repo.clone()),
        update_use_case: UpdateMerchant::new(repo.clone()),
        delete_use_case: DeleteMerchant::new(repo.clone()),
        
    };

    routes!(create_merchant::create_merchant_handler,
        get_merchant::get_merchant_handler,
        update_merchant::update_merchant_handler,
        delete_merchant::delete_merchant_handler)
        .with_state(state)
}

fn merchant_list(repo: Arc<dyn MerchantRepository>) -> UtoipaMethodRouter {
    routes!(list_merchants::list_merchants_handler)
        .with_state(ListMerchants::new(repo.clone()))
}