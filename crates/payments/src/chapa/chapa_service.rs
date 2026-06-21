use crate::chapa::chapa_webhook::ChapaWebhook;
use hulu_core::payment_gateway_error::PaymentGatewayError;

pub struct ChapaService {
    client: reqwest::Client,
}

impl ChapaService {
    pub async fn send_webhook(
        &self,
        url: &str,
        webhook: ChapaWebhook,
    ) -> Result<(), PaymentGatewayError> {
        let response = self
            .client
            .post(url)
            .json(&webhook)
            .send()
            .await
            .map_err(|_| PaymentGatewayError::RequestFailed)?;
        let status = response.status();
        if !status.is_success() {
            return Err(PaymentGatewayError::RequestFailed);
        }
        Ok(())
    }
}
