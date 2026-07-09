use crate::application::{
    CreateApiKey, CreateMerchant, DeleteApiKey, DeleteMerchant, GetMerchant, ListApiKeys,
    ListMerchants, UpdateApiKey, UpdateMerchant,
};

#[derive(Clone)]
pub struct MerchantState {
    pub create_use_case: CreateMerchant,
    pub get_use_case: GetMerchant,
    pub update_use_case: UpdateMerchant,
    pub delete_use_case: DeleteMerchant,
    pub list_merchants_use_case: ListMerchants,
    pub create_apikey_use_case: CreateApiKey,
    pub list_apikeys_use_case: ListApiKeys,
    pub update_apikey_use_case: UpdateApiKey,
    pub delete_apikey_use_case: DeleteApiKey,
}
