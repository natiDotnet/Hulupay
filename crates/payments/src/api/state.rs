use crate::application::{CreatePaymentProvider, GetPaymentProvider, UpdatePaymentProvider, DeletePaymentProvider, ListPaymentProviders};
use crate::infrastructure::ArifPayProvider;

#[derive(Clone)]
pub struct PaymentsState {
    pub arifpay_provider: ArifPayProvider,
    pub create_payment_provider: CreatePaymentProvider,
    pub get_payment_provider: GetPaymentProvider,
    pub update_payment_provider: UpdatePaymentProvider,
    pub delete_payment_provider: DeletePaymentProvider,
    pub list_payment_providers: ListPaymentProviders,
}
