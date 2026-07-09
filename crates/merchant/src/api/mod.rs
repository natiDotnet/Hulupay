mod create_apikey;
mod create_merchant;
mod delete_apikey;
mod delete_merchant;
mod get_merchant;
mod list_apikeys;
mod list_merchants;
mod middleware;
mod state;
mod update_apikey;
mod update_merchant;

pub use state::MerchantState;

use crate::application::{
    CreateApiKey, CreateMerchant, DeleteApiKey, DeleteMerchant, GetMerchant, ListApiKeys,
    ListMerchants, UpdateApiKey, UpdateMerchant,
};
use auth::Role;
use auth::api::middleware::AuthRouterExt;
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
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

impl FromRef<MerchantState> for CreateApiKey {
    fn from_ref(state: &MerchantState) -> Self {
        state.create_apikey_use_case.clone()
    }
}

impl FromRef<MerchantState> for ListApiKeys {
    fn from_ref(state: &MerchantState) -> Self {
        state.list_apikeys_use_case.clone()
    }
}

impl FromRef<MerchantState> for UpdateApiKey {
    fn from_ref(state: &MerchantState) -> Self {
        state.update_apikey_use_case.clone()
    }
}

impl FromRef<MerchantState> for DeleteApiKey {
    fn from_ref(state: &MerchantState) -> Self {
        state.delete_apikey_use_case.clone()
    }
}

pub fn router(db: &DatabaseConnection) -> OpenApiRouter {
    let state = MerchantState {
        create_use_case: CreateMerchant::new(db.clone()),
        get_use_case: GetMerchant::new(db.clone()),
        update_use_case: UpdateMerchant::new(db.clone()),
        delete_use_case: DeleteMerchant::new(db.clone()),
        list_merchants_use_case: ListMerchants::new(db.clone()),
        create_apikey_use_case: CreateApiKey::new(db.clone()),
        list_apikeys_use_case: ListApiKeys::new(db.clone()),
        update_apikey_use_case: UpdateApiKey::new(db.clone()),
        delete_apikey_use_case: DeleteApiKey::new(db.clone()),
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
        .routes(routes!(
            create_apikey::create_apikey_handler,
            list_apikeys::list_apikeys_handler
        ))
        .routes(routes!(
            delete_apikey::delete_apikey_handler,
            update_apikey::update_apikey_handler
        ))
        .require_role(Role::MasterAdmin)
        .with_state(state)
}
