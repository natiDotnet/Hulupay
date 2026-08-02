use crate::application::cache_service::CacheService;
use crate::application::merchants::get_merchant::get_merchant;
use crate::application::routing::engine::RoutingEngine;
use crate::domain::merchant_config::MerchantConfig;
use crate::domain::payment_order::PaymentOrder;
use crate::domain::payment_provider::PaymentProvider;
use crate::domain::payment_status::{PaymentStatus, TxDirection, TxStatus};
use crate::domain::payment_transaction::PaymentTransaction;
use crate::domain::payments::payment_callback::PaymentCallback;
use crate::domain::payments::payment_customer::PaymentCustomer;
use crate::domain::payments::payment_item::PaymentItem;
use crate::domain::provider::Provider;
use crate::{ProviderEngine, domain};
use async_trait::async_trait;
use hulu_core::create_checkout::CreateCheckout;
use hulu_core::gateway_response::CheckoutResponse;
use hulu_core::hulu_error::HuluError;
use hulu_core::payment_gateway_error::PaymentGatewayError;
use hulu_core::payment_request;
use hulu_core::request_context::RequestContext;
use payment_request::PaymentRequest;
use rust_decimal::Decimal;
use serde_json::json;
use std::sync::Arc;
use toasty::Db;
use tracing::debug;
use uuid::Uuid;

#[derive(Clone)]
pub struct CreateCheckoutHandler {
    db: Db,
    cache: Arc<dyn CacheService>,
    payment_engine: ProviderEngine,
    routing_engine: RoutingEngine,
}

impl CreateCheckoutHandler {
    pub fn new(
        db: Db,
        cache: Arc<dyn CacheService>,
        payment_engine: ProviderEngine,
        routing_engine: RoutingEngine,
    ) -> Self {
        Self {
            db,
            cache,
            payment_engine,
            routing_engine,
        }
    }
}
#[async_trait]
impl CreateCheckout for CreateCheckoutHandler {
    async fn execute(
        &self,
        context: &RequestContext,
        payload: PaymentRequest,
    ) -> Result<CheckoutResponse, HuluError> {
        debug!(?context, "Creating checkout");
        let merchant = get_merchant(&self.db, self.cache.as_ref(), &context.merchant)
            .await
            .ok_or(PaymentGatewayError::MerchantNotFound)?;

        // Smart routing: resolve an ordered list of providers, try each on failure.
        let provider_ids = self
            .routing_engine
            .resolve(merchant.id, &payload)
            .await
            .map_err(|_| HuluError::ProviderNotFound)?;

        // Build (gateway, merchant_config) pairs for each resolved provider, preserving order.
        // Fetch all valid configs for these providers (replaces inner_join).
        let mut all_configs = Vec::new();
        for pid in &provider_ids {
            let cfgs = MerchantConfig::filter(
                MerchantConfig::fields()
                    .merchant_id()
                    .eq(merchant.id)
                    .and(MerchantConfig::fields().provider_id().eq(*pid))
                    .and(MerchantConfig::fields().is_active().eq(true)),
            )
            .exec(&mut self.db.clone())
            .await
            .map_err(|_| HuluError::InternalServerError)?;
            all_configs.extend(cfgs);
        }

        // 2. Map them to your gateways
        let mut candidates = Vec::new();
        for cfg in all_configs {
            if let Some(gateway) = self.payment_engine.get_provider_by_id(cfg.provider_id).await {
                candidates.push((gateway, cfg));
            }
        }
        let (provider, merchant_config) = candidates
            .into_iter()
            .next()
            .ok_or(HuluError::ProviderNotFound)?;

        let request_provider = self
            .payment_engine
            .get_provider(None, Some(&context.provider.into()))
            .await
            .ok_or(HuluError::ProviderNotFound)?;

        let apikey_header = request_provider.get_apikey_name();
        let result = provider
            .checkout(context, &payload, apikey_header, merchant_config.config)
            .await;

        // Begin a Toasty transaction for the multi-row insert.
        let mut db_clone = self.db.clone();
        let mut txn = db_clone
            .transaction()
            .await
            .map_err(|_| HuluError::InternalServerError)?;

        let now = crate::util::now_jiff();
        let order = toasty::create!(PaymentOrder {
            merchant_id: merchant.id,
            customer_id: Uuid::now_v7(),
            request_provider: crate::domain::provider::Provider::from(context.provider),
            order_ref: payload.payment.reference.clone(),
            amount: payload.payment.amount,
            currency: payload.payment.currency.clone(),
            status: PaymentStatus::Pending,
            provider: Provider::ArifPay,
            idempotency_key: payload.payment.reference.clone(),
            retry_count: 0,
            created_at: now,
            updated_at: now,
        })
        .exec(&mut txn)
        .await
        .map_err(|_| HuluError::InternalServerError)?;

        // Build the success/failed tx row.
        let success_tx_id = Uuid::now_v7();
        let now = crate::util::now_jiff();

        let mut provider_tx_id: Option<String> = None;
        let mut provider_response: Option<serde_json::Value> = None;

        let gateway_response = match &result {
            Ok(data) => {
                provider_tx_id = Some(data.reference.clone());
                provider_response = data.row_response.clone();

                Some(CheckoutResponse {
                    checkout_url: data.checkout_url.clone(),
                    reference: data.reference.clone(),
                    amount: order.amount,
                })
            }

            Err(PaymentGatewayError::ProviderError {
                status_code,
                message,
                errors,
            }) => {
                provider_response = Some(json!({
                    "status_code": status_code,
                    "message": message,
                    "errors": errors,
                }));

                None
            }

            Err(_) => None,
        };

        toasty::create!(PaymentTransaction {
            id: success_tx_id,
            payment_order_id: order.id,
            provider: order.provider.clone(),
            provider_tx_id,
            direction: TxDirection::Charge,
            amount: order.amount,
            currency: order.currency.clone(),
            status: TxStatus::Pending,
            provider_response,
            created_at: now,
            updated_at: now,
        })
        .exec(&mut txn)
        .await
        .map_err(|_| HuluError::InternalServerError)?;

        save_info(&mut txn, order.id, &payload).await?;

        txn.commit()
            .await
            .map_err(|_| HuluError::InternalServerError)?;

        match result {
            Ok(_) => Ok(gateway_response.unwrap()),
            Err(e) => Err(e.into()),
        }
    }
}
async fn save_info(
    txn: &mut toasty::db::Transaction<'_>,
    order_id: Uuid,
    info: &PaymentRequest,
) -> Result<(), HuluError> {
    toasty::create!(PaymentCustomer {
        name: info.customer.name.clone(),
        email: info.customer.email.clone(),
        phone: info.customer.phone.clone(),
        account_number: None,
        payment_order_id: order_id,
    })
    .exec(txn)
    .await
    .map_err(|_| HuluError::InternalServerError)?;

    toasty::create!(PaymentCallback {
        cancel_url: info.callbacks.cancel_url.clone(),
        success_url: info.callbacks.success_url.clone(),
        notify_url: info.callbacks.notify_url.clone(),
        error_url: info.callbacks.error_url.clone(),
        payment_order_id: order_id,
    })
    .exec(txn)
    .await
    .map_err(|_| HuluError::InternalServerError)?;

    for item in info.items.clone().iter() {
        toasty::create!(PaymentItem {
            name: item.name.clone(),
            payment_order_id: order_id,
            description: item.description.clone(),
            image: item.image.clone(),
            quantity: item.quantity,
            unit_price: item.price,
            total_price: item.price * Decimal::from(item.quantity),
        })
        .exec(txn)
        .await
        .map_err(|_| HuluError::InternalServerError)?;
    }
    Ok(())
}
