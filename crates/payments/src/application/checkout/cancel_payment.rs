use crate::ProviderEngine;
use crate::application::cache_service::CacheService;
use crate::application::merchants::get_merchant::get_merchant;
use crate::domain::merchant_config::MerchantConfig;
use crate::domain::payment_order::PaymentOrder;
use crate::domain::payment_provider::PaymentProvider;
use crate::domain::payment_status::{PaymentStatus, TxDirection, TxStatus};
use crate::domain::payment_transaction::PaymentTransaction;
use async_trait::async_trait;
use hulu_core::create_checkout::CancelPayment;
use hulu_core::hulu_error::HuluError;
use hulu_core::payment_gateway_error::PaymentGatewayError;
use std::sync::Arc;
use toasty::Db;
use uuid::Uuid;

pub struct CancelPaymentHandler {
    db: Db,
    cache: Arc<dyn CacheService>,
    payment_engine: ProviderEngine,
}

impl CancelPaymentHandler {
    pub fn new(db: Db, cache: Arc<dyn CacheService>, payment_engine: ProviderEngine) -> Self {
        Self {
            db,
            cache,
            payment_engine,
        }
    }
}
#[async_trait]
impl CancelPayment for CancelPaymentHandler {
    async fn execute(&self, merchant: Uuid, reference: &str) -> Result<(), HuluError> {
        let mut db = self.db.clone();
        let merchant = get_merchant(&self.db, self.cache.as_ref(), merchant)
            .await
            .ok_or(PaymentGatewayError::MerchantNotFound)?;

        let provider = self
            .payment_engine
            .get_provider(Some(merchant.id), None)
            .await
            .ok_or(HuluError::ProviderNotFound)?;

        // Resolve the provider row by code, then find the merchant's active config.
        let provider_row = PaymentProvider::filter(
            PaymentProvider::fields()
                .code()
                .eq(provider.get_name().to_string()),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(|_| PaymentGatewayError::ProviderNotFound)?
        .ok_or(PaymentGatewayError::ProviderNotFound)?;

        let merchant_config = MerchantConfig::filter(
            MerchantConfig::fields()
                .merchant_id()
                .eq(merchant.id)
                .and(MerchantConfig::fields().provider_id().eq(provider_row.id))
                .and(MerchantConfig::fields().is_active().eq(true)),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(|_| PaymentGatewayError::ProviderNotFound)?
        .ok_or(PaymentGatewayError::ProviderNotFound)?;

        // Find the transaction by provider_tx_id, then load its order separately.
        let transaction = PaymentTransaction::filter(
            PaymentTransaction::fields()
                .provider_tx_id()
                .eq(reference.to_string()),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(|_| PaymentGatewayError::InternalServerError)?
        .ok_or(PaymentGatewayError::ProviderNotFound)?;

        let mut order = PaymentOrder::filter_by_id(transaction.payment_order_id)
            .first()
            .exec(&mut db)
            .await
            .map_err(|_| PaymentGatewayError::InternalServerError)?
            .ok_or(PaymentGatewayError::ProviderNotFound)?;

        if transaction.status == TxStatus::Success || order.status == PaymentStatus::Completed {
            return Err(HuluError::PaymentAlreadyCompleted);
        }
        let result = provider.cancel(reference, merchant_config.config).await?;

        toasty::update!(order {
            status: PaymentStatus::Cancelled,
            updated_at: crate::util::now_jiff(),
        })
        .exec(&mut db)
        .await
        .map_err(|_| PaymentGatewayError::InternalServerError)?;

        let now = crate::util::now_jiff();
        toasty::create!(PaymentTransaction {
            status: TxStatus::Failed,
            amount: order.amount,
            payment_order_id: order.id,
            provider_tx_id: None,
            direction: TxDirection::Charge,
            currency: order.currency.clone(),
            provider: crate::domain::provider::Provider::from(provider.get_name()),
            updated_at: now,
            provider_response: None,
            created_at: now,
        })
        .exec(&mut db)
        .await
        .map_err(|_| PaymentGatewayError::InternalServerError)?;
        Ok(result)
    }
}
