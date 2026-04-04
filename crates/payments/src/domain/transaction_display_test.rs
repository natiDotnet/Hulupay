use crate::TransactionStatus;
use std::fmt::Display;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_status_display() {
        assert_eq!(TransactionStatus::Pending.to_string(), "Pending");
        assert_eq!(TransactionStatus::Initialized.to_string(), "Initialized");
        assert_eq!(TransactionStatus::AwaitingConfirmation.to_string(), "AwaitingConfirmation");
        assert_eq!(TransactionStatus::Completed.to_string(), "Completed");
        assert_eq!(TransactionStatus::Failed.to_string(), "Failed");
        assert_eq!(TransactionStatus::Refunded.to_string(), "Refunded");
    }
}