use strum_macros::{Display, EnumString};
use thiserror::Error;

#[derive(Clone, Eq, PartialEq, Debug, Display, EnumString, toasty::Embed)]
#[column(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
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

impl From<hulu_core::payment_gateway::PaymentStatus> for PaymentStatus {
    fn from(value: hulu_core::payment_gateway::PaymentStatus) -> Self {
        match value {
            hulu_core::payment_gateway::PaymentStatus::Success => Self::Completed,
            hulu_core::payment_gateway::PaymentStatus::Failed => Self::Failed,
            hulu_core::payment_gateway::PaymentStatus::Pending => Self::Pending,
            hulu_core::payment_gateway::PaymentStatus::Cancelled => Self::Cancelled,
            hulu_core::payment_gateway::PaymentStatus::Refunding => Self::RefundPending,
            hulu_core::payment_gateway::PaymentStatus::Refunded => Self::Refunded,
            hulu_core::payment_gateway::PaymentStatus::Reversed => Self::Refunded,
        }
    }
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

#[derive(Clone, Eq, PartialEq, Debug, Display, EnumString, toasty::Embed)]
#[column(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum TxStatus {
    Pending,
    Success,
    Failed,
}

#[derive(Clone, Eq, PartialEq, Debug, toasty::Embed)]
#[column(rename_all = "UPPERCASE")]
pub enum TxDirection {
    Charge,
    Refund,
}
