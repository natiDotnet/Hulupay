use crate::application::cache_service::CacheService;
use crate::application::payment_gateway::ApiStatus;
use crate::application::PaymentGatewayError;
use crate::{domain, InitializePaymentCommand, ProviderEngine};
use rust_decimal::Decimal;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct InitializePaymentRequest {
    pub merchant_id: Uuid,
    pub phone: String,
    pub email: String,
    pub amount: Decimal,
    pub currency: String,
    pub provider: domain::provider::Provider,
    pub nonce: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct InitializePaymentResponse {
    pub status_code: u16,
    pub status: ApiStatus,
    pub message: String,
    pub checkout_url: Option<String>,
    pub provider_reference: Option<String>,
}
#[derive(Clone)]
pub struct InitiatePayment {
    db: DatabaseConnection,
    cache: Arc<dyn CacheService>,
    payment_engine: ProviderEngine,
}

impl InitiatePayment {
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
    pub async fn execute(
        &self,
        payload: InitializePaymentRequest,
    ) -> anyhow::Result<InitializePaymentResponse> {
        let provider = self
            .payment_engine
            .get_provider(payload.merchant_id, payload.provider)
            .ok_or(PaymentGatewayError::RequestFailed)?;

        let cmd = InitializePaymentCommand {
            merchant_id: payload.merchant_id,
            phone: payload.phone,
            email: payload.email,
            amount: payload.amount,
            currency: payload.currency,
        };
        let result = provider
            .initialize_payment(cmd)
            .await
            .map_err(|err| anyhow::anyhow!(err))?;

        Ok(InitializePaymentResponse {
            status_code: 200,
            status: ApiStatus::Success,
            message: "".to_string(),
            checkout_url: Some(result.checkout_url),
            provider_reference: Some(result.provider_reference),
        })
    }

    // pub async fn execute(
    //     &self,
    //     payload: InitializePaymentRequest,
    // ) -> anyhow::Result<InitializePaymentResponse> {
    //     // ── Stage 1: Idempotency — Redis fast path ────────────────────────────────
    //     let cache_key = format!("{}:{}", payload.merchant_id, &payload.nonce);
    //     if let Some(cached) =
    //         cache_get!(self.cache.as_ref(), InitializePaymentResponse, &cache_key).await?
    //     {
    //         return Ok(cached);
    //     }
    //     debug!(?cache_key, "test debug");
    //
    //     if payload.amount <= Decimal::ZERO {
    //         return Err(anyhow::anyhow!("amount must be greater than zero"));
    //     }
    //
    //     let order = PaymentOrders::find()
    //         .filter(payment_order::Column::MerchantId.eq(payload.merchant_id))
    //         .filter(payment_order::Column::IdempotencyKey.eq(&payload.nonce))
    //         .one(&self.db)
    //         .await?;
    //
    //     let order = match order {
    //         None => {
    //             if payload.amount <= Decimal::ZERO {
    //                 return Err(anyhow!("amount must be positive"));
    //             }
    //             let order = payment_order::ActiveModel {
    //                 merchant_id: Set(payload.merchant_id),
    //                 customer_id: Set(Uuid::now_v7()),
    //                 order_ref: Set(payload.nonce.clone()),
    //                 amount: Set(payload.amount),
    //                 currency: Set(payload.currency.clone()),
    //                 status: Set(PaymentStatus::Pending),
    //                 provider: Set(payload.provider.clone()),
    //                 idempotency_key: Set(payload.nonce.clone()),
    //                 retry_count: Set(0),
    //                 ..Default::default()
    //             }
    //             .insert(&self.db)
    //             .await?;
    //
    //             order
    //         }
    //         Some(existing) => existing,
    //     };
    //     match order.status {
    //         // Already completed or refunded — return the terminal state,
    //         // the client just didn't receive the response last time
    //         PaymentStatus::Completed | PaymentStatus::Refunded | PaymentStatus::Cancelled => {
    //             let response = InitializePaymentResponse {
    //                 status_code: 400,
    //                 status: ApiStatus::Failure,
    //                 message: "payment already completed or refunded or cancelled".to_string(),
    //                 provider_reference: None,
    //                 checkout_url: None,
    //             };
    //             cache_set!(self.cache.as_ref(), &cache_key, &response).await?;
    //             return Ok(response);
    //         }
    //         // Still in flight — fall through and try again with a new tx row
    //         PaymentStatus::Pending | PaymentStatus::Failed => {}
    //         // Processing means a tx is in flight — tell the client to wait
    //         PaymentStatus::Processing => {
    //             return Ok(InitializePaymentResponse {
    //                 status: ApiStatus::Success,
    //                 status_code: 200,
    //                 message: "the payment is under process".to_string(),
    //                 provider_reference: Some(order.order_ref),
    //                 checkout_url: None,
    //             });
    //         }
    //         _ => {
    //             return Err(anyhow!(format!(
    //                 "order in unexpected state: {:?}",
    //                 order.status
    //             )));
    //         }
    //     }
    //
    //     // ── Stage 3: Look at the last transaction to understand what happened ─────
    //     // Find the most recent transaction for this order. Its status tells us
    //     // whether the last attempt reached the provider or died locally.
    //     let last_tx = PaymentTransactions::find()
    //         .filter(payment_transaction::Column::PaymentOrderId.eq(order.id))
    //         .filter(payment_transaction::Column::Direction.eq(TxDirection::Charge))
    //         .order_by_desc(payment_transaction::Column::CreatedAt)
    //         .one(&self.db)
    //         .await?;
    //     let provider = self
    //         .payment_engine
    //         .get_provider(payload.merchant_id, payload.provider)
    //         // .await
    //         .ok_or(PaymentGatewayError::RequestFailed)?;
    //
    //     // If the last tx has a provider_tx_id but order is still Pending/Failed,
    //     // it means the provider accepted the charge but our webhook never arrived.
    //     // Poll the provider directly before creating a new transaction.
    //     if let Some(tx) = &last_tx
    //         && let Some(provider_tx_id) = &tx.provider_tx_id
    //     {
    //         // if let Some(ref provider_tx_id) = tx.provider_tx_id {
    //         // let provider = providers.resolve(payload.merchant_id, &order.provider)?;
    //         let verify = provider.verify_payment(provider_tx_id).await;
    //
    //         if let Ok(charge) = verify {
    //             let settled = charge.success;
    //
    //             if settled {
    //                 // Provider already has the money — transition order to Complete,
    //                 // create an immutable success tx row, no new charge attempt
    //                 let success_tx_id = Uuid::now_v7();
    //                 let now = Utc::now();
    //
    //                 payment_transaction::ActiveModel {
    //                     id: Set(success_tx_id),
    //                     payment_order_id: Set(order.id),
    //                     provider: Set(order.provider.clone()),
    //                     provider_tx_id: Set(Some(provider_tx_id.clone())),
    //                     direction: Set(TxDirection::Charge),
    //                     amount: Set(order.amount),
    //                     currency: Set(order.currency.clone()),
    //                     status: Set(TxStatus::Success),
    //                     provider_response: Set(json!(charge)),
    //                     created_at: Set(now),
    //                     updated_at: Set(now),
    //                 }
    //                 .insert(&self.db)
    //                 .await?;
    //
    //                 let mut order = order.into_active_model();
    //                 order.status = Set(PaymentStatus::Completed);
    //                 order.updated_at = Set(now);
    //
    //                 order.save(&self.db).await?;
    //
    //                 // state.event_bus.publish(
    //                 //     merchant.id,
    //                 //     PaymentEvent::Completed {
    //                 //         payment_id: order.id,
    //                 //         merchant_id: merchant.id,
    //                 //         occurred_at: now,
    //                 //     },
    //                 // );
    //
    //                 let response = InitializePaymentResponse {
    //                     provider_reference: provider_tx_id.to_string(),
    //                     // status: PaymentStatus::Completed,
    //                     checkout_url: None,
    //                 };
    //                 cache_set!(self.cache.as_ref(), &cache_key, &response).await?;
    //                 return Ok(response);
    //             }
    //             // Provider says not settled yet — fall through to new attempt
    //         }
    //         // }
    //     }
    //
    //     // ── Stage 4: Create a NEW transaction row for this attempt ────────────────
    //     // Never mutate a previous transaction row. Each row is an immutable receipt.
    //     // attempt number = number of existing charge rows + 1
    //     let attempt_number = PaymentTransactions::find()
    //         .filter(payment_transaction::Column::PaymentOrderId.eq(order.id))
    //         .filter(payment_transaction::Column::Direction.eq(TxDirection::Charge))
    //         .count(&self.db)
    //         .await? as i32
    //         + 1;
    //
    //     let tx_id = Uuid::now_v7();
    //     let now = Utc::now();
    //
    //     // Written before the provider call — provider_tx_id is None.
    //     // If we crash here, the watchdog sees a Pending tx with no provider_tx_id
    //     // and knows an attempt was in-flight.
    //     payment_transaction::ActiveModel {
    //         id: Set(tx_id),
    //         payment_order_id: Set(order.id),
    //         provider: Set(order.provider.clone()),
    //         provider_tx_id: Set(None),
    //         direction: Set(TxDirection::Charge),
    //         amount: Set(order.amount),
    //         currency: Set(order.currency.clone()),
    //         status: Set(TxStatus::Pending),
    //         provider_response: Set(serde_json::Value::Null),
    //         created_at: Set(now),
    //         updated_at: Set(now),
    //     }
    //     .insert(&self.db)
    //     .await?;
    //
    //     let cmd = InitializePaymentCommand {
    //         merchant_id: payload.merchant_id,
    //         phone: payload.phone,
    //         email: payload.email,
    //         amount: payload.amount,
    //         currency: payload.currency,
    //     };
    //     let result = provider.initialize_payment(cmd).await;
    //     //     .map_err(|e| {
    //     //     eprintln!("Error initializing payment: {:?}", e);
    //     //     anyhow!(e)
    //     // });
    //
    //     // ── Stage 6: Insert a NEW completed/failed tx — never touch the pending one
    //     match result {
    //         Ok(charge) => {
    //             // The pending tx row (no provider_tx_id) stays as-is — it records
    //             // that an attempt was dispatched. Now insert a success row with
    //             // the real provider_tx_id the provider returned.
    //             payment_transaction::ActiveModel {
    //                 id: Set(Uuid::now_v7()),
    //                 payment_order_id: Set(order.id),
    //                 provider: Set(order.provider.clone()),
    //                 provider_tx_id: Set(Some(charge.provider_reference.clone())),
    //                 direction: Set(TxDirection::Charge),
    //                 amount: Set(order.amount),
    //                 currency: Set(order.currency.clone()),
    //                 // Still Pending — the webhook will insert the final Success tx
    //                 status: Set(TxStatus::Pending),
    //                 provider_response: Set(json!(charge)),
    //                 created_at: Set(Utc::now()),
    //                 updated_at: Set(Utc::now()),
    //             }
    //             .insert(&self.db)
    //             .await?;
    //
    //             let mut order = order.into_active_model();
    //             order.status = Set(PaymentStatus::Processing);
    //             order.order_ref = Set(charge.provider_reference.clone());
    //             order.updated_at = Set(Utc::now());
    //
    //             order.save(&self.db).await?.try_into_model()?;
    //             // payment_order::ActiveModel {
    //             //     id: Set(order.id),
    //             //     status: Set(PaymentStatus::Processing),
    //             //     updated_at: Set(Utc::now()),
    //             //     ..Default::default()
    //             // }
    //             // .update(&self.db)
    //             // .await?;
    //
    //             // state.event_bus.publish(merchant.id, PaymentEvent::Processing {
    //             //     payment_id:     order.id,
    //             //     merchant_id:    merchant.id,
    //             //     provider_tx_id: charge.provider_tx_id.clone(),
    //             //     occurred_at:    Utc::now(),
    //             // });
    //
    //             let response = InitializePaymentResponse {
    //                 status_code: 200,
    //                 status: ApiStatus::Success,
    //                 message: "payment initiated successfully".to_string(),
    //                 provider_reference: Some(charge.provider_reference),
    //                 checkout_url: Some(charge.checkout_url.clone()),
    //             };
    //             cache_set!(self.cache.as_ref(), &cache_key, &response).await?;
    //             Ok(response)
    //         }
    //         Err(e) => {
    //             // Insert a Failed tx row —   record of this attempt's outcome.
    //             // The pending tx row stays untouched as evidence of dispatch.
    //             payment_transaction::ActiveModel {
    //                 id: Set(Uuid::now_v7()),
    //                 payment_order_id: Set(order.id),
    //                 provider: Set(order.provider.clone()),
    //                 provider_tx_id: Set(None),
    //                 direction: Set(TxDirection::Charge),
    //                 amount: Set(order.amount),
    //                 currency: Set(order.currency.clone()),
    //                 status: Set(TxStatus::Failed),
    //                 provider_response: Set(json!({
    //                     "error":   e.to_string(),
    //                     "attempt": attempt_number,
    //                 })),
    //                 created_at: Set(Utc::now()),
    //                 updated_at: Set(Utc::now()),
    //             }
    //             .insert(&self.db)
    //             .await?;
    //
    //             if e.is_retryable() {
    //                 payment_order::ActiveModel {
    //                     id: Set(order.id),
    //                     status: Set(PaymentStatus::Failed),
    //                     retry_count: Set(order.retry_count + 1),
    //                     updated_at: Set(Utc::now()),
    //                     ..Default::default()
    //                 }
    //                 .save(&self.db)
    //                 .await?;
    //
    //                 // state.event_bus.publish(merchant.id, PaymentEvent::Retrying {
    //                 //     payment_id:  order.id,
    //                 //     merchant_id: merchant.id,
    //                 //     attempt:     attempt_number,
    //                 //     occurred_at: Utc::now(),
    //                 // });
    //
    //                 Ok(InitializePaymentResponse {
    //                     status_code: 400,
    //                     status: ApiStatus::Failure,
    //                     message: "payment failed".to_string(),
    //                     provider_reference: None,
    //                     checkout_url: None,
    //                 })
    //             } else {
    //                 payment_order::ActiveModel {
    //                     id: Set(order.id),
    //                     status: Set(PaymentStatus::Failed),
    //                     updated_at: Set(Utc::now()),
    //                     ..Default::default()
    //                 }
    //                 .update(&self.db)
    //                 .await?;
    //
    //                 // state.event_bus.publish(merchant.id, PaymentEvent::Failed {
    //                 //     payment_id:  order.id,
    //                 //     merchant_id: merchant.id,
    //                 //     reason:      e.to_string(),
    //                 //     retryable:   false,
    //                 //     occurred_at: Utc::now(),
    //                 // });
    //
    //                 Err(anyhow!(e))
    //             }
    //         }
    //     }
    // }
}
