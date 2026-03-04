mod state;
mod create_merchant;
mod get_merchant;
mod update_merchant;
mod delete_merchant;
mod list_merchants;
mod middleware;

pub use state::MerchantState;
pub use middleware::{authentication, authorization, AuthorizationPolicy, AuthRouterExt};

use auth::Role;
use axum::extract::FromRef;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::application::{CreateMerchant, GetMerchant, UpdateMerchant, DeleteMerchant, ListMerchants, MerchantRepository};
use crate::infrastructure::MerchantRepositoryPostgres;

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

pub fn router(pool: Pool<Postgres>) -> OpenApiRouter {
    let repo: Arc<dyn MerchantRepository> = Arc::new(MerchantRepositoryPostgres::new(pool));
    
    let state = MerchantState {
        create_use_case: CreateMerchant::new(repo.clone()),
        get_use_case: GetMerchant::new(repo.clone()),
        update_use_case: UpdateMerchant::new(repo.clone()),
        delete_use_case: DeleteMerchant::new(repo.clone()),
        list_merchants_use_case: ListMerchants::new(repo.clone()),
    };

    OpenApiRouter::new()
        .routes(routes!(create_merchant::create_merchant_handler, list_merchants::list_merchants_handler))
        .routes(routes!(get_merchant::get_merchant_handler, delete_merchant::delete_merchant_handler, update_merchant::update_merchant_handler))
        .require_role(Role::MasterAdmin)
        .with_state(state)
}
