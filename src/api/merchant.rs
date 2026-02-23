use utoipa_axum::router::UtoipaMethodRouterExt;
mod create_merchant;
mod get_merchant;
mod update_merchant;
mod delete_merchant;

use application::merchant::create_merchant::CreateMerchant;
use application::merchant::repository::MerchantRepository;
use std::sync::Arc;
use utoipa_axum::router::UtoipaMethodRouter;
use utoipa_axum::routes;
use application::merchant::get_merchant::GetMerchant;
use axum::extract::FromRef;
use application::merchant::delete_merchant::DeleteMerchant;
use application::merchant::update_merchant::UpdateMerchant;

#[derive(Clone)]
struct MerchantState {
    create_usecase: CreateMerchant,
    get_use_case: GetMerchant,
    update_use_case: UpdateMerchant,
    delete_use_case: DeleteMerchant,
}

impl FromRef<MerchantState> for CreateMerchant {
    fn from_ref(state: &MerchantState) -> Self {
        state.create_usecase.clone()
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

pub fn router(repo: Arc<dyn MerchantRepository>) -> UtoipaMethodRouter {
    let create_usecase = CreateMerchant::new(repo.clone());
    let get_use_case = GetMerchant::new(repo.clone());

    let state = MerchantState {
        create_usecase,
        get_use_case,
        update_use_case: UpdateMerchant::new(repo.clone()),
        delete_use_case: DeleteMerchant::new(repo.clone())
    };

    routes!(create_merchant::create_merchant_handler,
        get_merchant::get_merchant_handler,
        update_merchant::update_merchant_handler,
        delete_merchant::delete_merchant_handler)
        .with_state(state)
}