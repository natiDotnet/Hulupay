use crate::application::cache_service::CacheService;
use crate::application::merchants::get_merchant::get_merchant;
use crate::domain::payment_status::{PaymentStatus, TxDirection, TxStatus};
use crate::domain::{
    MerchantConfigs, PaymentOrders, PaymentTransactions, merchant_config, payment_provider,
};
use crate::{ProviderEngine, domain};
use async_trait::async_trait;
use chrono::Utc;
use hulu_core::create_checkout::CancelPayment;
use hulu_core::hulu_error::HuluError;
use hulu_core::payment_gateway_error::PaymentGatewayError;
use sea_orm::{ActiveModelTrait, QueryFilter};
use sea_orm::{ColumnTrait, IntoActiveModel, Set};
use sea_orm::{DatabaseConnection, EntityTrait};
use std::sync::Arc;

pub struct CancelPaymentHandler {
    db: DatabaseConnection,
    cache: Arc<dyn CacheService>,
    payment_engine: ProviderEngine,
}

impl CancelPaymentHandler {
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
impl CancelPayment for CancelPaymentHandler {
    async fn execute(&self, merchant: &str, reference: &str) -> Result<(), HuluError> {
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

        let (transaction, order) = PaymentTransactions::find()
            .filter(domain::payment_transaction::Column::ProviderTxId.eq(reference))
            .find_also_related(PaymentOrders)
            .one(&self.db)
            .await
            .map_err(|_| PaymentGatewayError::InternalServerError)?
            .ok_or(PaymentGatewayError::ProviderNotFound)?;
        let order = order.ok_or(PaymentGatewayError::ProviderNotFound)?;
        if transaction.status == TxStatus::Success || order.status == PaymentStatus::Completed {
            return Err(HuluError::PaymentAlreadyCompleted);
        }
        let result = provider.cancel(reference, merchant_config.config).await?;

        let mut order = order.into_active_model();
        order.status = Set(PaymentStatus::Cancelled);

        domain::payment_transaction::ActiveModel {
            status: Set(TxStatus::Failed),
            amount: order.amount.clone(),
            payment_order_id: order.id.clone(),
            provider_tx_id: Set(None),
            direction: Set(TxDirection::Charge),
            currency: order.currency.clone(),
            provider: Set(provider.get_name().into()),
            updated_at: Set(Utc::now()),
            provider_response: Set(None),
            ..Default::default()
        }
        .insert(&self.db)
        .await
        .map_err(|_| PaymentGatewayError::InternalServerError)?;
        order
            .save(&self.db)
            .await
            .map_err(|_| PaymentGatewayError::InternalServerError)?;
        Ok(result)
    }
}
