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
use crate::api::middleware::AuthRouterExt;
use application::auth::token::TokenService;
use application::merchant::create_merchant::CreateMerchant;
use application::merchant::delete_merchant::DeleteMerchant;
use application::merchant::get_merchant::GetMerchant;
use application::merchant::list_merchants::ListMerchants;
use application::merchant::repository::MerchantRepository;
use application::merchant::update_merchant::UpdateMerchant;
use axum::extract::FromRef;
use domain::user::Role;
use infrastructure::persistence::merchant_repository_impl::MerchantRepositoryPostgres;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use utoipa_axum::routes;

#[derive(Clone)]
pub struct MerchantState {
    create_use_case: CreateMerchant,
    get_use_case: GetMerchant,
    update_use_case: UpdateMerchant,
    delete_use_case: DeleteMerchant,
    list_merchants_use_case: ListMerchants,
    // pub token_service: Arc<dyn TokenService>,
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

// impl FromRef<MerchantState> for Arc<dyn TokenService> {
//     fn from_ref(state: &MerchantState) -> Self {
//         state.token_service.clone()
//     }
// }

pub fn router(pool: Pool<Postgres>) -> OpenApiRouter {

    let repo: Arc<dyn MerchantRepository> = Arc::new(MerchantRepositoryPostgres::new(pool));
    // let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    // let token_service: Arc<dyn TokenService> = Arc::new(JwtTokenService::new(jwt_secret));
    let state = MerchantState {
        create_use_case: CreateMerchant::new(repo.clone()),
        get_use_case: GetMerchant::new(repo.clone()),
        update_use_case: UpdateMerchant::new(repo.clone()),
        delete_use_case: DeleteMerchant::new(repo.clone()),
        list_merchants_use_case: ListMerchants::new(repo.clone()),
    };

    OpenApiRouter::new()
        .routes(routes!(create_merchant_handler, list_merchants_handler))
        .routes(routes!(get_merchant_handler, delete_merchant_handler, update_merchant_handler))
        .require_role(Role::MasterAdmin)
        .with_state(state)

}