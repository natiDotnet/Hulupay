use crate::merchant::create_merchant::__path_create_merchant_handler;
use crate::merchant::delete_merchant::__path_delete_merchant_handler;
use crate::merchant::get_merchant::__path_get_merchant_handler;
use crate::merchant::list_merchants::__path_list_merchants_handler;
use crate::merchant::update_merchant::__path_update_merchant_handler;
use utoipa_axum::router::OpenApiRouter;
mod create_merchant;
mod get_merchant;
mod update_merchant;
mod delete_merchant;
mod list_merchants;

use crate::api::merchant::create_merchant::create_merchant_handler;
use crate::api::merchant::delete_merchant::delete_merchant_handler;
use crate::api::merchant::get_merchant::get_merchant_handler;
use crate::api::merchant::list_merchants::list_merchants_handler;
use crate::api::merchant::update_merchant::update_merchant_handler;
use application::merchant::create_merchant::CreateMerchant;
use application::merchant::delete_merchant::DeleteMerchant;
use application::merchant::get_merchant::GetMerchant;
use application::merchant::list_merchants::ListMerchants;
use application::merchant::repository::MerchantRepository;
use application::merchant::update_merchant::UpdateMerchant;
use axum::extract::FromRef;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_axum::routes;

#[derive(Clone)]
struct MerchantState {
    create_use_case: CreateMerchant,
    get_use_case: GetMerchant,
    update_use_case: UpdateMerchant,
    delete_use_case: DeleteMerchant,
    list_merchants_use_case: ListMerchants
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

impl FromRef<MerchantState> for ListMerchants {
    fn from_ref(state: &MerchantState) -> Self {
        state.list_merchants_use_case.clone()
    }
}

pub fn router(repo: Arc<dyn MerchantRepository>) -> OpenApiRouter {

    let state = MerchantState {
        create_use_case: CreateMerchant::new(repo.clone()),
        get_use_case: GetMerchant::new(repo.clone()),
        update_use_case: UpdateMerchant::new(repo.clone()),
        delete_use_case: DeleteMerchant::new(repo.clone()),
        list_merchants_use_case: ListMerchants::new(repo.clone())

    };

    OpenApiRouter::new()
        .routes(routes!(create_merchant_handler, list_merchants_handler))
        .routes(routes!(get_merchant_handler, delete_merchant_handler, update_merchant_handler))
        .with_state(state)

}