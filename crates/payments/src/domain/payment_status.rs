// domain/src/state_machine.rs

use sea_orm::sea_query::StringLen;
use sea_orm::{DeriveActiveEnum, EnumIter};
use thiserror::Error;

// #[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, serde::Serialize, serde::Deserialize)]
// #[sqlx(type_name = "payment_status", rename_all = "snake_case")]
#[derive(EnumIter, DeriveActiveEnum, Clone, Eq, PartialEq, Debug)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "UPPERCASE"
)]
pub enum PaymentStatus {
    Initiated,
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
    RefundPending,
    Refunded,
}

#[derive(Debug, Error)]
pub enum TransitionError {
    #[error("illegal transition from {from:?} to {to:?}")]
    Illegal {
        from: PaymentStatus,
        to: PaymentStatus,
    },
    #[error("payment is in a terminal state: {0:?}")]
    Terminal(PaymentStatus),
}

impl PaymentStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Cancelled | Self::Refunded
        )
    }

    // The entire state machine is this one function.
    // Every other module calls this — nothing mutates status directly.
    pub fn transition(&self, to: &PaymentStatus) -> Result<(), TransitionError> {
        if self.is_terminal() {
            return Err(TransitionError::Terminal(self.clone()));
        }
        let allowed = match self {
            Self::Initiated => vec![Self::Pending],
            Self::Pending => vec![Self::Processing, Self::Cancelled],
            Self::Processing => vec![Self::Completed, Self::Failed],
            Self::Failed => vec![Self::Pending], // retry path
            Self::Completed => vec![Self::RefundPending],
            Self::RefundPending => vec![Self::Refunded],
            _ => vec![],
        };
        if allowed.contains(to) {
            Ok(())
        } else {
            Err(TransitionError::Illegal {
                from: self.clone(),
                to: to.clone(),
            })
        }
    }
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Eq, PartialEq, Debug)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "UPPERCASE"
)]
pub enum TxStatus {
    Pending,
    Success,
    Failed,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Eq, PartialEq, Debug)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "UPPERCASE"
)]
pub enum TxDirection {
    Charge,
    Refund,
}
