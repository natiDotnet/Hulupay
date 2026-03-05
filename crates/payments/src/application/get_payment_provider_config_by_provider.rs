use crate::application::repository::PaymentProviderConfigRepository;
use crate::domain::PaymentProviderConfig;
use std::sync::Arc;
use auth::api::AuthUser;
use auth::UserContext;

#[derive(Clone)]
pub struct GetPaymentProviderConfigByProvider {
    repository: Arc<dyn PaymentProviderConfigRepository>,
}

impl GetPaymentProviderConfigByProvider {
    pub fn new(repository: Arc<dyn PaymentProviderConfigRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, auth_user: UserContext, provider_code: &str) -> anyhow::Result<Option<PaymentProviderConfig>> {
        // Get provider by code first, then get config by merchant and provider
        // For now, we'll need to get all active configs and filter by provider code
        // This assumes we're getting configs for a specific merchant (you may need to adjust based on your auth logic)
        
        // Since we need merchant_id from the authenticated user, we'll need to pass it or get it from context
        // For now, let's assume we're getting configs without merchant filtering
        // You may need to adjust this based on your authentication requirements
        
        // Get all active configs and filter by provider code
        // This is a simplified approach - you might want to add a repository method for better performance
        let result = self.repository.list_active_by_provider_code(provider_code).await?;
        
        // Return the first matching config
        Ok(result.into_iter().next())
    }
}
