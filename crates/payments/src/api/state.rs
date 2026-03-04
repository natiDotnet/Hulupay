use crate::infrastructure::ArifPayProvider;

#[derive(Clone)]
pub struct PaymentsState {
    pub arifpay_provider: ArifPayProvider,
}
