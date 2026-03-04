use crate::application::{CreateMerchant, GetMerchant, UpdateMerchant, DeleteMerchant, ListMerchants};

#[derive(Clone)]
pub struct MerchantState {
    pub create_use_case: CreateMerchant,
    pub get_use_case: GetMerchant,
    pub update_use_case: UpdateMerchant,
    pub delete_use_case: DeleteMerchant,
    pub list_merchants_use_case: ListMerchants,
}
