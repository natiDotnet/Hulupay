use crate::domain::payments::payment_customer::PaymentCustomer;
use crate::domain::payment_order::PaymentOrder;
use crate::application::payment_provider::list_payment_providers::PaginatedResponse;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use toasty::Db;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PaymentItem {
    pub reference: String,
    #[schema(value_type = f64)]
    pub amount: Decimal,
    pub provider: String,
    pub status: String,
    pub time: String,
    pub phone_number: String,
    pub email: String,
}

#[derive(Clone)]
pub struct ListPayments {
    db: Db,
}

impl ListPayments {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        merchant_id: Uuid,
        page: u64,
        page_size: u64,
    ) -> anyhow::Result<PaginatedResponse<PaymentItem>> {
        let mut db = self.db.clone();

        let all = PaymentOrder::filter(PaymentOrder::fields().merchant_id().eq(merchant_id))
            .exec(&mut db)
            .await?;
        let total = all.len() as u64;

        let offset = if page > 1 { (page - 1) * page_size } else { 0 };
        let orders = PaymentOrder::filter(PaymentOrder::fields().merchant_id().eq(merchant_id))
            .limit(page_size as usize)
            .offset(offset as usize)
            .exec(&mut db)
            .await?;

        let mut items = Vec::new();

        for order in orders {
            let customer_opt = PaymentCustomer::filter(
                PaymentCustomer::fields().payment_order_id().eq(order.id)
            )
            .first()
            .exec(&mut db)
            .await
            .ok()
            .flatten();

            let (phone, email) = match customer_opt {
                Some(c) => (c.phone.clone(), c.email.clone()),
                None => ("".to_string(), "".to_string()),
            };

            items.push(PaymentItem {
                reference: order.order_ref,
                amount: order.amount,
                provider: order.provider.to_string(),
                status: order.status.to_string(),
                time: order.created_at.to_string(),
                phone_number: phone,
                email,
            });
        }

        Ok(PaginatedResponse {
            items,
            total,
            page,
            page_size,
        })
    }
}
