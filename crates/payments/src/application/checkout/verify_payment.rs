use crate::application::cache_service::CacheService;
use crate::application::merchants::get_merchant::get_merchant;
use crate::domain::merchant_config::MerchantConfig;
use crate::domain::payment_order::PaymentOrder;
use crate::domain::payment_provider::PaymentProvider;
use crate::domain::payment_status::{PaymentStatus, TxStatus};
use crate::domain::payment_transaction::PaymentTransaction;
use crate::ProviderEngine;
use async_trait::async_trait;
use hulu_core::create_checkout::VerifyPayment;
use hulu_core::gateway_response::VerifyResponse;
use hulu_core::hulu_error::HuluError;
use hulu_core::payment_gateway_error::PaymentGatewayError;
use std::sync::Arc;
use toasty::Db;
use uuid::Uuid;

pub struct VerifyPaymentHandler {
    db: Db,
    cache: Arc<dyn CacheService>,
    payment_engine: ProviderEngine,
}

impl VerifyPaymentHandler {
    pub fn new(db: Db, cache: Arc<dyn CacheService>, payment_engine: ProviderEngine) -> Self {
        Self {
            db,
            cache,
            payment_engine,
        }
    }
}
#[async_trait]
impl VerifyPayment for VerifyPaymentHandler {
    async fn execute(&self, merchant: Uuid, reference: &str) -> Result<VerifyResponse, HuluError> {
        let mut db = self.db.clone();
        let merchant = get_merchant(&self.db, self.cache.as_ref(), merchant)
            .await
            .ok_or(PaymentGatewayError::MerchantNotFound)?;

        let provider = self
            .payment_engine
            .get_provider(Some(merchant.id), None)
            .await
            .ok_or(HuluError::ProviderNotFound)?;

        // Resolve provider row by code, then find merchant's active config.
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

        let result = provider.verify(reference, merchant_config.config).await?;

        // Find transaction by provider_tx_id, then load order separately.
        let mut transaction = PaymentTransaction::filter(
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

        let tx_status: TxStatus = result.status.clone().parse().unwrap();
        let order_status: PaymentStatus = result.status.clone().parse().unwrap();

        toasty::update!(order {
            status: order_status,
        })
        .exec(&mut db)
        .await
        .map_err(|_| PaymentGatewayError::InternalServerError)?;

        toasty::update!(transaction { status: tx_status })
            .exec(&mut db)
            .await
            .map_err(|_| PaymentGatewayError::InternalServerError)?;

        Ok(result)
    }
}
