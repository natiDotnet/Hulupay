use crate::application::cache_service::CacheService;
use crate::application::merchants::get_merchant::get_merchant;
use crate::domain::{
    MerchantConfigs, PaymentOrders, PaymentTransactions, merchant_config, payment_provider,
};
use crate::{ProviderEngine, domain};
use async_trait::async_trait;
use hulu_core::create_checkout::VerifyPayment;
use hulu_core::gateway_response::VerifyResponse;
use hulu_core::hulu_error::HuluError;
use hulu_core::payment_gateway_error::PaymentGatewayError;
use sea_orm::{ActiveModelTrait, QueryFilter};
use sea_orm::{ColumnTrait, IntoActiveModel, Set};
use sea_orm::{DatabaseConnection, EntityTrait};
use std::sync::Arc;

pub struct VerifyPaymentHandler {
    db: DatabaseConnection,
    cache: Arc<dyn CacheService>,
    payment_engine: ProviderEngine,
}

impl VerifyPaymentHandler {
    pub fn new(
        db: DatabaseConnection,
        cache: Arc<dyn CacheService>,
        payment_engine: ProviderEngine,
    ) -> Self {
        Self {
            db,
            cache,
            payment_engine,
        }
    }
}
#[async_trait]
impl VerifyPayment for VerifyPaymentHandler {
    async fn execute(&self, merchant: &str, reference: &str) -> Result<VerifyResponse, HuluError> {
        let merchant = get_merchant(&self.db, self.cache.as_ref(), merchant)
            .await
            .ok_or(PaymentGatewayError::MerchantNotFound)?;

        let provider = self
            .payment_engine
            .get_provider(Some(merchant.id), None)
            .await
            .ok_or(HuluError::ProviderNotFound)?;

        let merchant_config = MerchantConfigs::find()
            .inner_join(payment_provider::Entity)
            .filter(merchant_config::Column::MerchantId.eq(merchant.id))
            .filter(payment_provider::Column::Code.eq(provider.get_name().to_string()))
            .filter(merchant_config::Column::IsActive.eq(true))
            .one(&self.db)
            .await
            .map_err(|_| PaymentGatewayError::ProviderNotFound)?
            .ok_or(PaymentGatewayError::ProviderNotFound)?;

        let result = provider.verify(reference, merchant_config.config).await?;

        let (transaction, order) = PaymentTransactions::find()
            .filter(domain::payment_transaction::Column::ProviderTxId.eq(reference))
            .find_also_related(PaymentOrders)
            .one(&self.db)
            .await
            .map_err(|_| PaymentGatewayError::InternalServerError)?
            .ok_or(PaymentGatewayError::ProviderNotFound)?;

        let mut order = order
            .ok_or(PaymentGatewayError::ProviderNotFound)?
            .into_active_model();

        let mut tnx = transaction.into_active_model();
        tnx.status = Set(result.status.clone().parse().unwrap());

        order.status = Set(result.status.clone().parse().unwrap());

        order
            .save(&self.db)
            .await
            .map_err(|_| PaymentGatewayError::InternalServerError)?;
        tnx.save(&self.db)
            .await
            .map_err(|_| PaymentGatewayError::InternalServerError)?;
        Ok(result)
    }
}
