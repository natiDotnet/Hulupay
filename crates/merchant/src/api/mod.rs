mod create_merchant;
mod delete_merchant;
mod get_merchant;
mod list_merchants;
mod middleware;
mod state;
mod update_merchant;

pub use state::MerchantState;

use crate::application::{
    CreateMerchant, DeleteMerchant, GetMerchant, ListMerchants, MerchantRepository, UpdateMerchant,
};
use crate::infrastructure::MerchantRepositoryPostgres;
use auth::api::middleware::AuthRouterExt;
use auth::Role;
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

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

pub fn router(pool: Pool<Postgres>, db: &DatabaseConnection) -> OpenApiRouter {
    let repo: Arc<dyn MerchantRepository> = Arc::new(MerchantRepositoryPostgres::new(pool));

    let state = MerchantState {
        create_use_case: CreateMerchant::new(db.clone()),
        get_use_case: GetMerchant::new(db.clone()),
        update_use_case: UpdateMerchant::new(db.clone()),
        delete_use_case: DeleteMerchant::new(db.clone()),
        list_merchants_use_case: ListMerchants::new(db.clone()),
    };

    OpenApiRouter::new()
        .routes(routes!(
            create_merchant::create_merchant_handler,
            list_merchants::list_merchants_handler
        ))
        .routes(routes!(
            get_merchant::get_merchant_handler,
            delete_merchant::delete_merchant_handler,
            update_merchant::update_merchant_handler
        ))
        .require_role(Role::MasterAdmin)
        .with_state(state)
}
