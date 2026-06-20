use crate::application::cache_service::CacheService;
use crate::application::merchants::get_merchant::get_merchant;
use crate::domain::payment_status::{PaymentStatus, TxDirection, TxStatus};
use crate::domain::provider::Provider;
use crate::domain::{
    merchant_config, payment_order, payment_provider, payment_transaction, MerchantConfigs,
};
use crate::{domain, ProviderEngine};
use async_trait::async_trait;
use chrono::Utc;
use hulu_core::create_checkout::CreateCheckout;
use hulu_core::gateway_response::CheckoutResponse;
use hulu_core::hulu_error::HuluError;
use hulu_core::payment_gateway_error::PaymentGatewayError;
use hulu_core::payment_request;
use hulu_core::request_context::RequestContext;
use payment_request::PaymentRequest;
use rust_decimal::Decimal;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct CreateCheckoutHandler {
    db: DatabaseConnection,
    cache: Arc<dyn CacheService>,
    payment_engine: ProviderEngine,
}

impl CreateCheckoutHandler {
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
impl CreateCheckout for CreateCheckoutHandler {
    async fn execute(
        &self,
        context: &RequestContext,
        payload: PaymentRequest,
    ) -> Result<CheckoutResponse, HuluError> {
        let merchant = get_merchant(&self.db, self.cache.as_ref(), &context.merchant)
            .await
            .ok_or(PaymentGatewayError::MerchantNotFound)?;

        let provider = self
            .payment_engine
            .get_provider(Some(merchant.id), None)
            .await
            .ok_or(HuluError::ProviderNotFound)?;

        let merchant_config: merchant_config::Model = MerchantConfigs::find()
            .inner_join(payment_provider::Entity)
            .filter(merchant_config::Column::MerchantId.eq(merchant.id))
            .filter(payment_provider::Column::Code.eq(provider.get_name().to_string()))
            .filter(merchant_config::Column::IsActive.eq(true))
            .one(&self.db)
            .await
            .map_err(|_| PaymentGatewayError::ProviderNotFound)?
            .ok_or(PaymentGatewayError::ProviderNotFound)?;

        let result = provider.checkout(&payload, merchant_config.config).await;

        let order = payment_order::ActiveModel {
            merchant_id: Set(merchant.id),
            customer_id: Set(Uuid::now_v7()),
            order_ref: Set(payload.payment.reference.clone()),
            amount: Set(payload.payment.amount),
            currency: Set(payload.payment.currency.clone()),
            status: Set(PaymentStatus::Pending),
            provider: Set(Provider::ArifPay),
            idempotency_key: Set(payload.payment.reference.clone()),
            retry_count: Set(0),
            ..Default::default()
        }
        .insert(&self.db)
        .await
        .map_err(|_| HuluError::InternalServerError)?;

        // Provider already has the money — transition order to Complete,
        // create an immutable success tx row, no new charge attempt
        let success_tx_id = Uuid::now_v7();
        let now = Utc::now();

        let mut transaction = payment_transaction::ActiveModel {
            id: Set(success_tx_id),
            payment_order_id: Set(order.id),
            provider: Set(order.provider.clone()),
            provider_tx_id: Set(None),
            direction: Set(TxDirection::Charge),
            amount: Set(order.amount),
            currency: Set(order.currency.clone()),
            status: Set(TxStatus::Pending),
            provider_response: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        self.save_info(order.id, &payload).await?;

        let gateway_response = match &result {
            Ok(data) => {
                transaction.provider_tx_id = Set(Some(data.reference.clone()));
                transaction.provider_response = Set(data.row_response.clone());

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
                transaction.provider_response = Set(Some(json!({
                    "status_code": status_code,
                    "message": message,
                    "errors": errors,
                })));

                None
            }

            Err(_) => None,
        };

        transaction
            .save(&self.db)
            .await
            .map_err(|_| HuluError::InternalServerError)?;

        match result {
            Ok(_) => Ok(gateway_response.unwrap()),
            Err(e) => Err(e.into()),
        }
    }
}
impl CreateCheckoutHandler {
    async fn save_info(&self, order_id: Uuid, info: &PaymentRequest) -> Result<(), HuluError> {
        domain::payments::payment_customer::ActiveModel {
            name: Set(info.customer.name.clone()),
            email: Set(info.customer.email.clone()),
            phone: Set(info.customer.phone.clone()),
            account_number: Set(None),
            payment_order_id: Set(order_id),
            ..Default::default()
        }
        .insert(&self.db)
        .await
        .map_err(|_| HuluError::InternalServerError)?;

        domain::payments::payment_callback::ActiveModel {
            cancel_url: Set(info.callbacks.cancel_url.clone()),
            success_url: Set(info.callbacks.success_url.clone()),
            notify_url: Set(info.callbacks.notify_url.clone()),
            payment_order_id: Set(order_id),
            ..Default::default()
        }
        .insert(&self.db)
        .await
        .map_err(|_| HuluError::InternalServerError)?;

        let items = info
            .items
            .clone()
            .iter()
            .map(|item| domain::payments::payment_item::ActiveModel {
                name: Set(item.name.clone()),
                payment_order_id: Set(order_id),
                description: Set(item.description.clone()),
                image: Set(item.image.clone()),
                quantity: Set(item.quantity),
                unit_price: Set(item.price),
                total_price: Set(item.price * Decimal::from(item.quantity)),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        domain::payments::payment_item::Entity::insert_many(items)
            .exec(&self.db)
            .await
            .map_err(|_| HuluError::InternalServerError)?;
        Ok(())
    }
}
